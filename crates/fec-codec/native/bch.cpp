/*
 * Copyright (c) 2017-2026 aff3ct
 * SPDX-License-Identifier: MIT
 *
 * Adapted from AFF3CT commit 0e8ecda8191cec97a6fe86a56de0d957b63b3b04,
 * src/Module/Decoder/BCH/Standard/Decoder_BCH_std.cpp. See AFF3CT-LICENSE.
 * Local adaptation: isolate the BCH kernel from StreamPU; bound ELP storage by
 * 2t+3 (the largest accessed iteration/degree) instead of the field order;
 * supply ETSI's generator and primitive polynomial; reverse MSB-first wire
 * order at the boundary; reject roots in shortened positions and recheck every
 * syndrome after correction. No modem behavior or FEC reliability is inferred.
 */
#include "fec-codec/src/lib.rs.h"
#include "native/bch.hpp"
#include <algorithm>
#include <array>
#include <sstream>
#include <stdexcept>
#include <vector>

namespace pnm_fec {
namespace {
constexpr int BCH_MESSAGE_BITS = 14232;
constexpr int BCH_WORD_BITS = 14400;
constexpr int BCH_PARITY_BITS = 168;
constexpr int BCH_CORRECTION_BITS = 12;
constexpr int BCH_FIELD_ORDER = 16383;

class BchKernel {
public:
    const int N = BCH_WORD_BITS;
    const int t = BCH_CORRECTION_BITS;
    const int t2 = 2 * BCH_CORRECTION_BITS;
    const int N_p2_1 = BCH_FIELD_ORDER;
    std::vector<int> alpha_to = std::vector<int>(BCH_FIELD_ORDER + 1);
    std::vector<int> index_of = std::vector<int>(BCH_FIELD_ORDER + 1, -1);
    std::vector<int> generator;
    std::vector<std::vector<int>> elp = std::vector<std::vector<int>>(27, std::vector<int>(27));
    std::vector<int> discrepancy = std::vector<int>(27);
    std::vector<int> l = std::vector<int>(27);
    std::vector<int> u_lu = std::vector<int>(27);
    std::vector<int> s = std::vector<int>(25);
    std::vector<int> loc = std::vector<int>(13);
    std::vector<int> reg = std::vector<int>(13);
    std::array<bool, 1> last_is_codeword{};

    BchKernel() {
        // ETSI EN302769 V1.2.1 Table4(b), g1..g12, increasing powers.
        const std::array<unsigned, 12> factors = {0x402b, 0x4941, 0x4647, 0x5591, 0x6b55, 0x6389, 0x6ce5, 0x4f21, 0x460f, 0x5a49, 0x5811, 0x65ef};
        generator = {1};
        for (const unsigned factor : factors) {
            std::vector<int> product(generator.size() + 14);
            for (std::size_t power = 0; power < generator.size(); ++power) {
                for (unsigned term = 0; term <= 14; ++term) {
                    product[power + term] ^= generator[power] & ((factor >> term) & 1U);
                }
            }
            generator = std::move(product);
        }
        int value = 1;
        for (int power = 0; power < BCH_FIELD_ORDER; ++power) {
            if (index_of[value] != -1) {
                throw std::invalid_argument("BCH field polynomial is not primitive");
            }
            alpha_to[power] = value;
            index_of[value] = power;
            value <<= 1;
            if (value & (1 << 14)) {
                value ^= factors[0];
            }
        }
        alpha_to[BCH_FIELD_ORDER] = 1;
        if (value != 1 || generator.size() != BCH_PARITY_BITS + 1) {
            throw std::invalid_argument("Invalid BCH field or generator");
        }
    }
    bool valid(const std::vector<int>& word) const {
        for (int root = 1; root <= t2; ++root) {
            int syndrome = 0;
            for (int position = 0; position < N; ++position) {
                if (word[position]) {
                    syndrome ^= alpha_to[(root * position) % BCH_FIELD_ORDER];
                }
            }
            if (syndrome != 0) {
                return false;
            }
        }
        return true;
    }
    int decode(int* Y_N, const std::size_t frame_id)
{
    int i, j, syn_error = 0;

    /* first form the syndromes */
    for (i = 1; i <= t2; i++)
    {
        s[i] = 0;
        for (j = 0; j < this->N; j++)
            if (Y_N[j] != 0) s[i] ^= alpha_to[(i * j) % this->N_p2_1];
        if (s[i] != 0) syn_error = 1; /* set error flag if non-zero syndrome */
        /* convert syndrome from polynomial form to index form  */
        s[i] = (int)index_of[s[i]];
    }

    this->last_is_codeword[frame_id] = !syn_error;

    if (syn_error)
    { /* if there are errors, try to correct them */
        /*
         * Compute the error location polynomial via the Berlekamp
         * iterative algorithm. Following the terminology of Lin and
         * Costello's book :   d[u] is the 'mu'th discrepancy, where
         * u='mu'+1 and 'mu' (the Greek letter!) is the step number
         * ranging from -1 to 2*this->t (see L&C),  l[u] is the degree of
         * the elp at that step, and u_l[u] is the difference between
         * the step number and the degree of the elp.
         */
        /* initialise table entries */
        discrepancy[0] = 0;    /* index form */
        discrepancy[1] = s[1]; /* index form */
        elp[0][0] = 0;         /* index form */
        elp[1][0] = 1;         /* polynomial form */
        for (i = 1; i < t2; i++)
        {
            elp[0][i] = -1; /* index form */
            elp[1][i] = 0;  /* polynomial form */
        }
        l[0] = 0;
        l[1] = 0;
        u_lu[0] = -1;
        u_lu[1] = 0;

        int q, u = 0;
        do
        {
            u++;

            if (discrepancy[u] == -1)
            {
                l[u + 1] = l[u];
                for (i = 0; i <= l[u]; i++)
                {
                    elp[u + 1][i] = elp[u][i];
                    elp[u][i] = (int)index_of[elp[u][i]];
                }
            }
            else
            { // search for words with greatest u_lu[q] for which d[q]!=0
                q = u - 1;
                while ((discrepancy[q] == -1) && (q > 0))
                    q--;
                /* have found first non-zero d[q]  */
                if (q > 0)
                {
                    j = q;
                    do
                    {
                        j--;
                        if ((discrepancy[j] != -1) && (u_lu[q] < u_lu[j])) q = j;
                    } while (j > 0);
                }

                /*
                 * have now found q such that d[u]!=0 and
                 * u_lu[q] is maximum
                 */
                /* store degree of new elp polynomial */
                if (l[u] > l[q] + u - q)
                    l[u + 1] = l[u];
                else
                    l[u + 1] = l[q] + u - q;

                /* form new elp(x) */
                for (i = 0; i < t2; i++)
                    elp[u + 1][i] = 0;
                for (i = 0; i <= l[q]; i++)
                    if (elp[q][i] != -1)
                        elp[u + 1][i + u - q] =
                          (int)alpha_to[(discrepancy[u] + this->N_p2_1 - discrepancy[q] + elp[q][i]) % this->N_p2_1];
                for (i = 0; i <= l[u]; i++)
                {
                    elp[u + 1][i] ^= elp[u][i];
                    elp[u][i] = (int)index_of[elp[u][i]];
                }
            }
            u_lu[u + 1] = u - l[u + 1];

            /* form (u+1)th discrepancy */
            if (u < t2)
            {
                /* no discrepancy computed on last iteration */
                if (s[u + 1] != -1)
                    discrepancy[u + 1] = (int)alpha_to[s[u + 1]];
                else
                    discrepancy[u + 1] = 0;

                for (i = 1; i <= l[u + 1]; i++)
                    if ((s[u + 1 - i] != -1) && (elp[u + 1][i] != 0))
                        discrepancy[u + 1] ^= alpha_to[(s[u + 1 - i] + index_of[elp[u + 1][i]]) % this->N_p2_1];
                /* put d[u+1] into index form */
                discrepancy[u + 1] = (int)index_of[discrepancy[u + 1]];
            }
        } while ((u < t2) && (l[u + 1] <= this->t));

        u++;
        if (l[u] <= this->t)
        { /* May correct errors */
            /* put elp into index form */
            for (i = 0; i <= l[u]; i++)
                elp[u][i] = (int)index_of[elp[u][i]];

            /* Chien search: find roots of the error location polynomial */
            for (i = 1; i <= l[u]; i++)
                reg[i] = elp[u][i];

            int count = 0;
            for (i = 1; i <= this->N_p2_1; i++)
            {
                q = 1;
                for (j = 1; j <= l[u]; j++)
                    if (reg[j] != -1)
                    {
                        reg[j] = (reg[j] + j) % this->N_p2_1;
                        q ^= alpha_to[reg[j]];
                    }
                if (!q)
                { /* store root and error
                   * location number indices */
                    if (static_cast<size_t>(count) >= loc.size())
                    {
                        std::stringstream message;
                        message << "The polynomial seems not to be primitive.";
                        throw std::invalid_argument(message.str());
                    }
                    loc[count++] = this->N_p2_1 - i;
                }
            }

            if (count == l[u])
            {
                this->last_is_codeword[frame_id] = true;

                /* no. roots = degree of elp hence <= this->t errors */
                for (i = 0; i < l[u]; i++)
                    if (loc[i] < this->N) Y_N[loc[i]] ^= 1;

                return 0;
            }
            else
                return 1;
        }
        else
            return 1;
    }
    else
        return 0;
}
};
}

rust::Vec<std::uint8_t> bch_encode(rust::Slice<const std::uint8_t> message) {
    if (message.size() != BCH_MESSAGE_BITS) {
        throw std::invalid_argument("Invalid BCH message length");
    }
    BchKernel kernel;
    std::vector<int> remainder(BCH_PARITY_BITS);
    // AFF3CT Encoder_BCH::__encode stores polynomial coefficients low-first.
    // The external protocol is MSB-first, message followed by parity.
    for (const auto bit : message) {
        if (bit > 1) {
            throw std::invalid_argument("Nonbinary BCH input");
        }
        const int feedback = bit ^ remainder[BCH_PARITY_BITS - 1];
        for (int power = BCH_PARITY_BITS - 1; power > 0; --power) {
            remainder[power] = remainder[power - 1] ^ (kernel.generator[power] & feedback);
        }
        remainder[0] = feedback & kernel.generator[0];
    }
    rust::Vec<std::uint8_t> result;
    result.reserve(BCH_WORD_BITS);
    for (const auto bit : message) {
        result.push_back(bit);
    }
    for (auto position = remainder.rbegin(); position != remainder.rend(); ++position) {
        result.push_back(static_cast<std::uint8_t>(*position));
    }
    return result;
}

BchDecoded bch_decode(rust::Slice<const std::uint8_t> codeword) {
    if (codeword.size() != BCH_WORD_BITS) {
        throw std::invalid_argument("Invalid BCH word length");
    }
    BchKernel kernel;
    std::vector<int> word(codeword.begin(), codeword.end());
    std::reverse(word.begin(), word.end());
    if (std::any_of(word.begin(), word.end(), [](int bit) { return bit < 0 || bit > 1; })) {
        throw std::invalid_argument("Nonbinary BCH input");
    }
    const auto original = word;
    const int status = kernel.decode(word.data(), 0);
    std::uint32_t corrected = 0;
    for (std::size_t position = 0; position < word.size(); ++position) {
        if (word[position] != original[position]) {
            ++corrected;
        }
    }
    // A root outside the transmitted shortened word is not a corrected symbol.
    const bool success = status == 0 && corrected <= BCH_CORRECTION_BITS && kernel.valid(word);
    BchDecoded result;
    result.success = success;
    result.corrected = success ? corrected : 0;
    const auto& returned = success ? word : original;
    for (auto position = returned.rbegin(); position != returned.rend(); ++position) {
        result.codeword.push_back(static_cast<std::uint8_t>(*position));
    }
    return result;
}
}
