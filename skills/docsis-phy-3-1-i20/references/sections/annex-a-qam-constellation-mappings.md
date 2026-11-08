# Annex A - QAM Constellation Mappings

Source: CM-SP-PHYv3.1-I20-230419
PDF pages: 197-206
Classification: Normative (explicit)

<!-- PDF_PAGE: 197 -->
Physical Layer Specification                                                        CM-SP-PHYv3.1-I20-230419


Annex A          QAM Constellation Mappings (Normative)
The CMTS MUST use the QAM constellation mappings given in this section for all downstream transmissions.
Downstream transmissions do not contain 8-QAM or 32-QAM constellations.
The CM MUST use the QAM constellation mappings given in this section for all upstream transmissions. Upstream
transmissions do not contain 8192-QAM and 16384-QAM constellations.
Sample code showing the bit to QAM constellation mapping is provided in [PHYv3.1 QAM].

A.1     QAM Constellations
The figures given below show the mapping of an m-tuple onto a (Real, Imaginary) point in the complex plane. The
horizontal axis is the real axis and the vertical axis is the imaginary axis.
The m-tuple is represented by:
        {y0 y1 … ym-1}
Mapping of the FEC encoded bitstreams to the m-tuples is described in the sections detailing downstream and
upstream transmissions.
Each m-tuple is represented as a hexadecimal number in all the constellation diagrams given below.




                                   Figure 99 - BPSK Constellation Mapping of {y0}




                                 Figure 100 - QPSK Constellation Mapping of {y0 y1}




04/19/23                                           CableLabs                                                 197

<!-- PDF_PAGE: 198 -->
CM-SP-PHYv3.1-I20-230419                            Data-Over-Cable Service Interface Specifications




                     Figure 101 - 8-QAM Constellation Mapping of {y0 y1 y2}




                   Figure 102 - 16-QAM Constellation Mapping of {y0 y1 y2 y3}




                  Figure 103 - 32-QAM Constellation Mapping of {y0 y1 y2 y3 y4}




198                                      CableLabs                                        04/19/23

<!-- PDF_PAGE: 199 -->
Physical Layer Specification                                                        CM-SP-PHYv3.1-I20-230419




                       Figure 104 - 64-QAM Constellation Mapping of {y0 y1 y2 y3 y4 y5}




                    Figure 105 - 128-QAM Constellation Mapping of {y0 y1 y2 y3 y4 y5 y6}
In order to reduce the size of the diagrams, only the first quadrant is shown for the larger constellations, namely,
256-QAM, 512-QAM, 1024-QAM, 2048-QAM, 4096-QAM, 8192-QAM and 16384-QAM. The mapping of the two
bits {y0 y1} of {y0 y1 … ym-1} is the same for all these QAM constellations, as illustrated in Figure 106.




04/19/23                                           CableLabs                                                   199

<!-- PDF_PAGE: 200 -->
CM-SP-PHYv3.1-I20-230419                                       Data-Over-Cable Service Interface Specifications




  Figure 106 - Mapping of Bits {y0 y1} of {y0 y1, …, ym-1} for Constellations with Only one Quadrant Defined
The mapping of the bits of {y2 y3, …, ym-1} to the first quadrant of the constellation is given in the figures below
for 256-QAM, 512-QAM, 1024-QAM, 2048-QAM, 4096-QAM, 8192-QAM and 16384-QAM. The mappings for
the other three quadrants are obtained by mirroring the first quadrant about the horizontal and vertical axis as
illustrated in the figure below. This figure shows only a (3x3) grid of points in each quadrant for illustration of the
above mentioned reflective property. However, this reflective mapping is applicable to any number of points in each
quadrant. Quadrant 1 is reflected about the vertical axis to get quadrant 2. Quadrant 1 is reflected about the
horizontal axis to get quadrant 4. Quadrant 2 is reflected about the horizontal axis to get quadrant 3.




         Figure 107 - Reflective Mapping of bits {y2 y3, ..., ym-1} for All Constellations (except BPSK)




200                                                 CableLabs                                               04/19/23

<!-- PDF_PAGE: 201 -->
Physical Layer Specification                                                 CM-SP-PHYv3.1-I20-230419




            Figure 108 - 256-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7} on to Quadrant 1




           Figure 109 - 512-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7 y8} on to Quadrant 1




04/19/23                                       CableLabs                                          201

<!-- PDF_PAGE: 202 -->
CM-SP-PHYv3.1-I20-230419                                Data-Over-Cable Service Interface Specifications




       Figure 110 - 1024-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7 y8 y9} on to Quadrant 1




      Figure 111 - 2048-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7 y8 y9 y10} on to Quadrant 1




202                                           CableLabs                                        04/19/23

<!-- PDF_PAGE: 203 -->
Physical Layer Specification                                                CM-SP-PHYv3.1-I20-230419




    Figure 112 - 4096-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7 y8 y9 y10 y11} on to Quadrant 1




04/19/23                                     CableLabs                                                 203

<!-- PDF_PAGE: 204 -->
CM-SP-PHYv3.1-I20-230419                                                                     Data-Over-Cable Service Interface Specifications




                   Figure 113 - 8192-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7 y8 y9 y10 y11 y12} on to Quadrant 1




204                                                           CableLabs                                                            04/19/23

<!-- PDF_PAGE: 205 -->
Physical Layer Specification                                                                                         CM-SP-PHYv3.1-I20-230419




                    Figure 114 - 16384-QAM Constellation Mapping of {y2 y3 y4 y5 y6 y7 y8 y9 y10 y11 y12 y13} on to Quadrant 1




04/19/23                                                          CableLabs                                                             205

<!-- PDF_PAGE: 206 -->
CM-SP-PHYv3.1-I20-230419                                    Data-Over-Cable Service Interface Specifications



A.2     QAM Constellation Scaling
The CM MUST scale real and imaginary axes of the constellations by the scaling factors given in column 3 of the
table below, to ensure that the mean square value of all QAM constellations are equal to 1.0.
The CMTS MUST scale real and imaginary axes of the constellations by the scaling factors given in column 3 of the
table below, to ensure that the mean square value of all QAM constellations are equal to 1.0.
                                 Table 54 - QAM Constellation Scaling Factors

                                  QAM               m                  Scaling
                               Constellation    Number of bits         Factor
                                BPSK             1                 1
                                QPSK             2

                                8-QAM            3

                                16-QAM           4

                                32-QAM           5

                                64-QAM           6

                                128-QAM          7

                                256-QAM          8

                                512-QAM          9

                                1024-QAM         10

                                2048-QAM         11

                                4096-QAM         12

                                8192-QAM         13

                                16384-QAM        14




206                                               CableLabs                                            04/19/23
