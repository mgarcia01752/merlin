# Appendix III - Upstream Time and Frequency Interleaver Sample C Code

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 214-214
Classification: Informative (explicit)

<!-- PDF_PAGE: 214 -->
CM-SP-PHYv3.1-I20-230419                                      Data-Over-Cable Service Interface Specifications


Appendix III              Upstream Time and Frequency Interleaver Sample C Code
                          (Informative)
The algorithm for generating the sequence of addresses (t, f) is described using the following C code segment:
Count_t = 0;
Count_f = 0;
Count_diagonal = 0;
for (idx_t=0;   idx_t<K;  idx_t++)
{
for (idx_f=0;   idx_f<L;  idx_f++)
{
Address.t = Bit_Reverse_Count(&Count_t, K);
Address.f = Bit_Reverse_Count(&Count_f, L);
Count_t = (Count_t + 1) % K1;
Count_f = (Count_f + 1) % L1;
}
Count_diagonal = (Count_diagonal + 1) % K1;
d = Bit_Reverse_Count(&Count_diagonal, K);
Count_t = Count_diagonal;
Count_f = 0;
}




214                                                CableLabs                                              04/19/23
