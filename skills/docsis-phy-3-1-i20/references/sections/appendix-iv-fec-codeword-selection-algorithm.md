# Appendix IV - FEC Codeword Selection Algorithm / Upstream Interleaver Sample C Code

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 215-217
Classification: Informative (explicit)

<!-- PDF_PAGE: 215 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


Appendix IV                FEC Codeword Selection Algorithm Upstream Time and
                           Frequency Interleaver Sample C Code (Informative)
If the CMTS scheduler wishes to grant a certain number of information bits, it needs to perform the opposite
(forward) calculation in order to determine how many codewords of what sizes are necessary to hold the desired
number of information bits. This is part of the process of determining the grant size. For informative purposes, the
script forward_calc.m is provided to show how the CMTS could perform this calculation. The variable finfo_size is
the input to this script.
% forward_calc script
% set values for codeword sizes
% total bits = size including parity
% info bits = information bits only
% thresholds - if more bits than threshold, shorten this cw instead of
% using a smaller one
% short codeword
SHORT_TOTAL_BITS = 1120;
SHORT_INFO_BITS = 840;
SHORT_PARITY_BITS = SHORT_TOTAL_BITS - SHORT_INFO_BITS;
SHORT_TOTAL_THRESH_BITS = SHORT_PARITY_BITS + 1;
SHORT_MIN_INFO_BITS = SHORT_INFO_BITS / 2;
% medium codeword
MED_TOTAL_BITS = 5940;
MED_INFO_BITS = 5040;
MED_PARITY_BITS = MED_TOTAL_BITS - MED_INFO_BITS;
MED_INFO_THRESH_BITS = 2521;
% long codeword
LONG_TOTAL_BITS = 16200;
LONG_INFO_BITS = 14400;
LONG_PARITY_BITS = LONG_TOTAL_BITS - LONG_INFO_BITS;
LONG_TOTAL_THRESH_BITS = 11881;
LONG_INFO_THRESH_BITS = 10081;
% variable finfo_size is input
% set finfo_size to desired input value in workspace
% initialize output variables
flong_cws = 0;
fshortened_long_cws = 0;
fmed_cws = 0;
fshortened_med_cws = 0;
fshort_cws = 0;
fshortened_short_cws = 0;
fother_shortened_cw_bits = 0;
fshortened_cw_bits = 0;
mac_padding = 0;
% intermediate variable to track type of last full codeword
flast_full_cw = '';
% now begin calculation
  bits_remaining = finfo_size;
  % if there are no bits at all, we don't want to give any grant - just let
  % everything fall through to zero codewords.
  % However, if we have a nonzero number of bits that is not enough to make
  % a min-size short codeword, we do want to give a grant but we are to
  % allow space for the CM to make a segment that will fill the min-size
  % short codeword.
  if (bits_remaining > 0 && bits_remaining < SHORT_MIN_INFO_BITS)
      mac_padding = SHORT_MIN_INFO_BITS - bits_remaining;
      bits_remaining = SHORT_MIN_INFO_BITS;
  end
    % make as many long cws as possible
    while bits_remaining ≥ LONG_INFO_BITS
        flong_cws = flong_cws + 1;




04/19/23                                           CableLabs                                                    215

<!-- PDF_PAGE: 216 -->
CM-SP-PHYv3.1-I20-230419                        Data-Over-Cable Service Interface Specifications


          bits_remaining = bits_remaining - LONG_INFO_BITS;
          flast_full_cw = 'Long';
      end
      % if remaining bits can make a shortened long cw, do so
      if bits_remaining ≥ LONG_INFO_THRESH_BITS
          fshortened_long_cws = 1;
          fshortened_cw_bits = bits_remaining;
          bits_remaining = 0;
      end
      % now make as many medium cws as possible
      while bits_remaining ≥ MED_INFO_BITS
          fmed_cws = fmed_cws + 1;
          bits_remaining = bits_remaining - MED_INFO_BITS;
          flast_full_cw = 'Medium';
      end
      % if remaining bits can make a shortened med cw, do so
      if bits_remaining ≥ MED_INFO_THRESH_BITS
          fshortened_med_cws = 1;
          fshortened_cw_bits = bits_remaining;
          bits_remaining = 0;
      end
      % now make as many short cws as possible
      while bits_remaining ≥ SHORT_INFO_BITS
          fshort_cws = fshort_cws + 1;
          bits_remaining = bits_remaining - SHORT_INFO_BITS;
          flast_full_cw = 'Short';
      end
      % if there are any bits left, finish with a shortened short cw
      if bits_remaining ≥ 1
          fshortened_short_cws = 1;
          % we need at least SHORT_MIN_INFO_BITS in a shortened short
          % codeword; if we don't have enough, we will borrow from the
          % immediately preceding full codeword, which will become another
          % shortened codeword.
          % Note that we will always borrow SHORT_MIN_INFO_BITS.
          if bits_remaining ≥ SHORT_MIN_INFO_BITS
               % no need to borrow bits
               fshortened_cw_bits = bits_remaining;
               bits_remaining = 0;
          else
               % identify type/size of last full codeword
               switch flast_full_cw
                   case 'Long'
                       % change last full cw to a shortened cw
                       flong_cws = flong_cws - 1;
                       fshortened_long_cws = fshortened_long_cws + 1;
                       % number of bits in that cw is reduced by
                       % SHORT_MIN_INFO_BITS
                       fother_shortened_cw_bits = LONG_INFO_BITS - ...
                           SHORT_MIN_INFO_BITS;
                       % put those bits plus bits_remaining into the last
                       % shortened cw
                 fshortened_cw_bits = SHORT_MIN_INFO_BITS +bits_remaining;
                       bits_remaining = 0;
                   case 'Medium'
                       % same steps as with long
                       fmed_cws = fmed_cws - 1;
                       fshortened_med_cws = fshortened_med_cws + 1;
                       fother_shortened_cw_bits = MED_INFO_BITS - ...
                           SHORT_MIN_INFO_BITS;
                       fshortened_cw_bits = SHORT_MIN_INFO_BITS +bits_remaining;
                       bits_remaining = 0;
                   case 'Short'



216                                     CableLabs                                     04/19/23

<!-- PDF_PAGE: 217 -->
Physical Layer Specification                                                       CM-SP-PHYv3.1-I20-230419


                          % also same as long
                          fshort_cws = fshort_cws - 1;
                          fshortened_short_cws = fshortened_short_cws + 1;
                          fother_shortened_cw_bits = SHORT_INFO_BITS - ...
                              SHORT_MIN_INFO_BITS;
                          fshortened_cw_bits = SHORT_MIN_INFO_BITS +bits_remaining;
                          bits_remaining = 0;
                 end
           end
     end
Tables showing the number and size of codewords to be used for grant sizes from 1 bit up to two full long
codewords are provided in [PHYv3.1 CODECHECK].




04/19/23                                          CableLabs                                                217
