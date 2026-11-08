# Appendix I - Downstream Frequency Interleaver Sample C Code

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 211-212
Classification: Informative (explicit)

<!-- PDF_PAGE: 211 -->
Physical Layer Specification                                                         CM-SP-PHYv3.1-I20-230419


Appendix I                 Downstream Frequency Interleaver Sample C Code
                           (Informative)
In the downstream frequency interleaver C code given below, X is the input data array, Y is the output data array,
and N is the size of each array. It has been written to illustrate each operation clearly, and as such it may not
necessarily be the most efficient implementation.
void Docsis_3_1_Freqeuncy_Interleaver ( int *X, int *Y, int N )
{
       int store[128][64];
       int i, j, K, k1, base;
       int RowCount, RowCount_BR;
       int ColumnCount, ColumnCount_BR;
       int Last_Column_Size;
       int column_rotate[64], rotated_column[128];
       // Number of rows is 128
       // Number of columns is K
       K = (int) (ceil (((double) N)/128.0));
       Last_Column_Size = N - ((K-1)*128);
       // Generate the column rotation array using the 6-bit LFSR
       column_rotate[0]=17;
       for (i=0; i<63; i++)
       {
                 int lsb;
                 lsb = column_rotate[i]&1;
               column_rotate[i+1] = (column_rotate[i] >> 1) ^ (lsb << 5) ^ (lsb << 4);
       }
       base = 0;
         // Save data in 2-D store with rows addressed in bit-reversed order
       for (RowCount=0; RowCount< 128; RowCount++)
       {
               RowCount_BR = 0;
               for (j=0; j<7; j++)
               {
                              RowCount_BR = RowCount_BR << 1;;
                              if (((RowCount >> j) & 1) == 1)
                       RowCount_BR++;
                 }
               if (RowCount_BR < Last_Column_Size)
               {
                              for (j=0; j<K; j++)
                              store[RowCount_BR][j] = X[base+j];
                              base += K;
               }
               else
               {
                              for (j=0; j<K-1; j++)
                              store[RowCount_BR][j] = X[base+j];
                              base += (K-1);
                 }
          }
       // Rotate columns 0 to K-2
       // Last column that could be partially filled is not rotated
       for (j=0; j<(K-1); j++)
       {
               for (i=0; i<128; i++)
                              rotated_column[(i + column_rotate[j]) & 0x7F] =
store[i][j];
               for (i=0; i<128; i++)
                              store[i][j] = rotated_column[i];
       }



04/19/23                                           CableLabs                                                    211

<!-- PDF_PAGE: 212 -->
CM-SP-PHYv3.1-I20-230419                      Data-Over-Cable Service Interface Specifications


       // Determine the number of bits k1 in ColumnCount
       k1 = 0;
       while ((1 << k1) < K)
              k1++;
       // Address columns in bit-reversed order, but only if bit-reversed number is
within range
       // Otherwise use non-bit-reversed version
       base = 0;
       for (ColumnCount=0; ColumnCount<K; ColumnCount++)
       {
              int ReadColumn;
              // Bit Reverse ColumnCount
              ColumnCount_BR = 0;
              for (i=0; i<k1; i++)
              {
                           ColumnCount_BR = ColumnCount_BR << 1;
                           if (((ColumnCount >> i) & 1) == 1)
                           ColumnCount_BR++;
              }
              if (ColumnCount_BR > (K-1)) // Read from ColumnCount
                           ReadColumn = ColumnCount;
              else
                           ReadColumn = ColumnCount_BR;
              if (ReadColumn == (K-1)) // Last column could be a partially-filled
column
              {
                           for (i=0; i<Last_Column_Size; i++)
                           Y[base+i] = store[i][ReadColumn];
                           base = base+Last_Column_Size;
              }
              else
              {
                           for (i=0; i<128; i++)
                           Y[base+i] = store[i][ReadColumn];
                           base = base+128;
              }
       }
       return;
}




212                                   CableLabs                                     04/19/23
