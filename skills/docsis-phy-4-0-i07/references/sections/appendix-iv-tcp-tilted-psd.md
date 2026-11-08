# Appendix IV - Calculations of Total Composite Power under Tilted PSD

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 154-158
Classification: Informative (explicit)

<!-- PDF_PAGE: 154 -->
CM-SP-PHYv4.0-I07-250627                                                                                         Data-Over-Cable Service Interface Specifications


Appendix IV                                           Calculations of Total Composite Power (TCP) Under a
                                                      Tilted Power Spectral Density (PSD)
Linear Equalizer
      𝑃𝑃𝑑𝑑𝑑𝑑 (f) = mf + b (dBmV/MHz) ; m ≡ slope, b ≡ intercept
is the linear function of power per MHz (i.e., power spectral density PSD per MHz) in decibel millivolt as a function
of frequency in MHz with slope m and intercept b. The Total Composite Power (TCP) in decibel millivolt (dBmV)
from the equalizer start frequency fstart to the equalizer stop frequency fstop is given by the logarithm of the integral of
PdB(f) as:
                                         𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                                                                                             10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
      𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 � � 10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑� = 10 log �                                                               �              � (𝑑𝑑𝑑𝑑𝑑𝑑𝑑𝑑)
                                                                                                               𝑚𝑚
                                                                                                               10 ln(10)
                                                                                                                             𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                       𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

where
      𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇 (𝑑𝑑𝑑𝑑) = 𝐿𝐿𝐿𝐿𝐿𝐿𝐿𝐿𝐿𝐿 @ 𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹 − 𝐿𝐿𝐿𝐿𝐿𝐿𝐿𝐿𝐿𝐿 @ 𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹
                                                  𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇
      m = 𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹 −𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹

b = Transmit Level @ Equalizer Start Frequency - Equalizer Slope * Equalizer Start Frequency -10 𝑙𝑙𝑙𝑙𝑙𝑙(6)
      and Transmit Level @ Equalizer Start Frequency = PSD per 6 MHz (dBmV/6 MHz) at that initial
      frequency.
                        10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10           𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠    10
                                                                  �𝑚𝑚𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 +𝑏𝑏�/10
                                                                                          10(𝑚𝑚𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 +𝑏𝑏)/10
The notation               𝑚𝑚               �                =      𝑚𝑚                  −      𝑚𝑚                   .
                           10
                              ln(10)            𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠        10 ln(10)                  10
                                                                                                   ln(10)




Cable Equalizer
      𝑃𝑃𝑑𝑑𝑑𝑑 (f) = m�𝑓𝑓 + b (dBmV/MHz) ; m ≡ slope, b ≡ intercept
is the linear function of power per √𝑀𝑀𝑀𝑀𝑀𝑀 (i.e., power spectral density PSD per √𝑀𝑀𝑀𝑀𝑀𝑀) in decibel millivolt as a
function of the square root of frequency in √𝑀𝑀𝑀𝑀𝑀𝑀 with slope m and intercept b. The total composite power TCP in
decibel millivolt (dBmV) from the equalizer start frequency fstart to the equalizer stop frequency fstop is given by the
logarithm of the integral of PdB(f) as:
                                            𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                                                                                                  2 · 10� 𝑚𝑚�𝑓𝑓+𝑏𝑏 �/10     𝑚𝑚                    𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
        𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 � � 10� 𝑚𝑚�𝑓𝑓+𝑏𝑏 �/10 𝑑𝑑𝑑𝑑� = 10 𝑙𝑙𝑙𝑙𝑙𝑙 �                                                                2     �10 ln(10) �𝑓𝑓 - 1��                � (𝑑𝑑𝑑𝑑𝑑𝑑𝑑𝑑)
                                                                                                                          𝑚𝑚
                                                                                                                        � 10 ln(10)�                             𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                         𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

where
                                                                      𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇
      𝑚𝑚 =
                 �𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹 − �𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹

b = Transmit Level @ Equalizer Start Frequency + Equalizer Tilt - 𝑚𝑚�𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹 -10 𝑙𝑙𝑙𝑙𝑙𝑙(6)
NOTE: Cable equalizers are designed to offset both the tilt and the shape associated with coaxial cable loss. A
      cable equalizer dB value equals the attenuation at the equalizer stop frequency given by:
                                                                                          𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇
      Cable EQ (dB) =
                                         1− �𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹⁄𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑆𝑆𝑆𝑆𝑆𝑆𝑆𝑆 𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹𝐹




154                                                                                             CableLabs                                                                  06/27/25
<!-- PDF_PAGE: 155 -->
Physical Layer Specification                                                                         CM-SP-PHYv4.0-I07-250627


Example
The previous formulas can be embedded in a spreadsheet to facilitate calculation of TCP and plots of equalizer
response and node or amplifier output, as shown in the following examples.
                                                                                    Node          L.E.
                                     Transmit Level (dBmV/6 MHz):                          37             32
Equalizer Direction (Node):                         Downstream           Equalizer Direction (Line Extender):    Downstream
Equalizer Type (Cable,Linear,Flat,Tilt,Sawtooth):        Linear          Equalizer Type (Cable,Linear):               Cable
Equalizer Low Frequency (MHz):                             111           Equalizer Low Frequency (MHz):                111
Equalizer High Frequency (MHz):                           1215           Equalizer High Frequency (MHz):              1215
Equalizer Tilt (dB/1104 MHz):                               21           Equalizer Tilt (dB/1104 MHz):                  21




                                                     Total Composite Power = 73.8 (dBmV)




                                                     Total Composite Power = 70.0 (dBmV)

Figure 36 - Graphs of an Example Node's Downstream Output Signal (top) with a 21 dB Linear Equalizer and
  an Example Line Extender Amplifier's Downstream Output Signal (bottom) with a 30 dB Cable Equalizer
The blue trace in each graph in Figure 36 is amplitude in dBmV/6 MHz, and the red trace is the equalizer
attenuation relative to the value at the highest (equalizer stop frequency) in dB. TCP = 73.8 dBmV for the top graph
of a 21 dB linear equalizer. TCP = 70.0 dBmV for the bottom graph of a 30 dB cable equalizer.
Of particular importance is managing TCP in adjusting levels for improved node signal to reflection interference
ratio or cable modem downstream receive level in full duplex operation or managing maximum TCP in extended
spectrum operation – for instance, operation to 1.8 GHz (or higher). Several approaches can be used to manage
signal carriage and TCP across a wide operating bandwidth, including a suitable constant tilt from the start of the



06/27/25                                                          CableLabs                                                  155
<!-- PDF_PAGE: 156 -->
CM-SP-PHYv4.0-I07-250627                                                                                                                 Data-Over-Cable Service Interface Specifications


downstream spectrum (fstart) to the upper frequency limit (fstop); unused gaps in the spectrum; a drop (“step” or
“stepdown”) in the operating levels above a certain frequency; multiple steps in operating levels at higher
frequencies; and so forth as depicted in the following figures.




                                                                                  Signal level




                                                                                                                                                                Signal level
 Signal level




                fstart         Frequency                        fstop                            fstart                    Frequency                    fstop                  fstart   Frequency        fstop


                Figure 37 - Examples of Ways to Manage Maximum TCP: Constant Tilt (left), Tilt with Single Stepdown
                                         (center), and Tilt with Multiple Stepdowns (right)

If an amplitude stepdown of D dB is introduced above frequency fD with the same slope, then
                                       10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10              𝑓𝑓𝐷𝐷                                           10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 �                        �                            + 10−𝐷𝐷/10                             �           �
                                          𝑚𝑚
                                          10 ln(10)
                                                                𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠                                        𝑚𝑚
                                                                                                                    10 ln(10)
                                                                                                                                    𝑓𝑓𝐷𝐷
The same approach can be used iteratively for piecewise continuous PSD segments (multiple steps).
For an example of Figure 37 center (tilt with a single stepdown), consider the TCP calculation for 72 dBmV and 71
dBmV in Section 7.2.5.9.1, Figure 13. The downstream power profile spans 108 MHz to 1218 MHz with 21 dB tilt
and a D dB stepdown at fD MHz.
To calculate the TCP for this profile, the slope m and intercept b are given by:
                          21
                m = 1218 −108 =0.0189
                b = 37 - 0.0189 * 108 -10 𝑙𝑙𝑙𝑙𝑙𝑙(6) =27.1752
                and
                                           (0.0189 𝑓𝑓+27.1752)                                              (0.0189 𝑓𝑓+27.1752)
                                       10           10                                            𝑓𝑓𝐷𝐷  − 10
                                                                                                         𝐷𝐷           10          𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 �                                                 �              + 10 10                     �            �
                                                0.0189
                                                 10      ln(10)                        𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠            0.0189
                                                                                                                 10 ln(10)
                                                                                                                                     𝑓𝑓𝐷𝐷

                                 (27.1752)
                             10     10                                                           (0.0189 𝑓𝑓)        𝑓𝑓𝐷𝐷                   𝐷𝐷      (0.0189 𝑓𝑓) 𝑓𝑓
                                                                                                                                                                𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                = 10 𝑙𝑙𝑙𝑙𝑙𝑙 � 0.0189            � +10 𝑙𝑙𝑙𝑙𝑙𝑙 �10                                     10      �                  + 10−10 10             10     �          �
                                10     ln(10)                                                                    𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠                                    𝑓𝑓𝐷𝐷

                                                  𝑓𝑓𝐷𝐷                𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠                          𝐷𝐷              𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠          𝑓𝑓𝐷𝐷
                = 50.78 + 10 𝑙𝑙𝑙𝑙𝑙𝑙 �10 529 − 10 529 + 10−10 �10 529 − 10 529 ��

which is the equation used for the above slope and intercept in Table 17.
                                                                   Signal level




                                                                                  fstart                                   Frequency             f1 f2 fstop

                                        Figure 38 - Example of a Gap in the Power Spectrum Lowering TCP

If the linearly tilted spectrum is continuous between fstart and fstop, then
                                       10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 �                 �              �
                                         𝑚𝑚
                                         10 ln(10)
                                                       𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠




156                                                                                                              CableLabs                                                                         06/27/25
<!-- PDF_PAGE: 157 -->
Physical Layer Specification                                                                                                               CM-SP-PHYv4.0-I07-250627


If there is a gap between f1 and f2 (e.g., an exclusion zone), then
                               10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10                  𝑓𝑓1              10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
    𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 �                                �                  +                  �           �
                                      𝑚𝑚
                                      10 ln(10)
                                                            𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠          𝑚𝑚
                                                                                  10 ln(10)
                                                                                                  𝑓𝑓2
Therefore, a summation of any number of gaps can be accommodated with such additional terms.

IV.1      Derivation of Amplifier Equalizer Total Composite Power (TCP) Formulas
Linear Equalizer
𝑃𝑃𝑑𝑑𝑑𝑑 (f) = mf + b (dBmV/MHz) ; m ≡ slope, b ≡ intercept
                                  𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

    𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 � � 10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑�
                                𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

Integration by variable substitution:
    ∫ 𝑒𝑒 𝑢𝑢 𝑑𝑑𝑑𝑑 = 𝑒𝑒 𝑢𝑢 and 𝑒𝑒 ln(10) = 10, so

    � 10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑 = � 𝑒𝑒 ln(10)(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑

    Let 𝑢𝑢 = ln(10)(𝑚𝑚𝑚𝑚 + 𝑏𝑏)/10
           𝑚𝑚
    𝑑𝑑𝑑𝑑 = 10 𝑙𝑙𝑙𝑙(10) 𝑑𝑑𝑑𝑑


                                                                           𝑚𝑚                    𝑒𝑒 𝑢𝑢            𝑒𝑒 ln(10)(𝑚𝑚𝑚𝑚+𝑏𝑏)/10       10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10
    � 𝑒𝑒 ln(10)(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑 = � 𝑒𝑒 𝑢𝑢 𝑑𝑑𝑑𝑑��10 𝑙𝑙𝑙𝑙(10)� =                               𝑚𝑚              =        𝑚𝑚                 =      𝑚𝑚
                                                                                             �10 𝑙𝑙𝑙𝑙(10)�            �10 𝑙𝑙𝑙𝑙(10)�             �10 𝑙𝑙𝑙𝑙(10)�

Therefore
                                         10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10
    � 10(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑 =                     𝑚𝑚
                                               �10 𝑙𝑙𝑙𝑙(10)�



Cable Equalizer

                                                                  �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 − �𝑓𝑓
    𝑃𝑃𝑑𝑑𝑑𝑑 (f) = 𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸𝐸 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇 �1 −                                           � + Transmit Level @ Equalizer Start Frequency
                                                             �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 − �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                                                                                                                                                                      𝑑𝑑𝑑𝑑𝑑𝑑𝑑𝑑
                                                                                                                                                          − 10 𝑙𝑙𝑙𝑙𝑙𝑙(6) �                     �
                                                                                                                                                                                       𝑀𝑀𝑀𝑀𝑀𝑀
                                                                                                                              𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇 �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
    = 𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇+ Transmit Level @ Equalizer Start Frequency −
                                                                                                                                  �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 − �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                                                                                                                                                 𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇
                                                                                                                                                            +                                  �𝑓𝑓
                                                                                                                                                                �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 − �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
where
             𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇
    𝑚𝑚 =                                        ≡ 𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
            �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 − �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
                                                                                                                                   𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇 �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠
    𝑏𝑏 = 𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇+ Transmit Level @ Equalizer Start Frequency −
                                                                                                                                        �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 − �𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

       = 𝐸𝐸𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞𝑞 𝑇𝑇𝑇𝑇𝑇𝑇𝑇𝑇+ Transmit Level @ Equalizer Start Frequency − m�𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠 ≡ 𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖




06/27/25                                                                                  CableLabs                                                                                       157
<!-- PDF_PAGE: 158 -->
CM-SP-PHYv4.0-I07-250627                                                                                         Data-Over-Cable Service Interface Specifications


Substituting the linear function slope and intercept into 𝑃𝑃𝑑𝑑𝑑𝑑 (f) yields:
      𝑃𝑃𝑑𝑑𝑑𝑑 (f) = m�𝑓𝑓 + b (dBmV/MHz)
Integrating 𝑃𝑃𝑑𝑑𝑑𝑑 (f) from the equalizer start frequency fstart to the equalizer stop frequency fstop yields:
                                      𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

      𝑇𝑇𝑇𝑇𝑇𝑇 = 10 𝑙𝑙𝑙𝑙𝑙𝑙 � � 10� 𝑚𝑚�𝑓𝑓+𝑏𝑏 �/10 𝑑𝑑𝑑𝑑�
                                  𝑓𝑓𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠𝑠

      Using 𝑒𝑒 ln(10) = 10

      � 10�𝑚𝑚�𝑓𝑓+𝑏𝑏�/10 𝑑𝑑𝑑𝑑 = � 𝑒𝑒 ln(10)�𝑚𝑚�𝑓𝑓+𝑏𝑏�/10 𝑑𝑑𝑑𝑑
                                  1
      Let 𝑢𝑢 = �𝑓𝑓 = 𝑓𝑓 2
                                                 𝑑𝑑
      Using the Chain Rule                            𝑓𝑓 𝑛𝑛 = 𝑛𝑛𝑓𝑓 𝑛𝑛−1
                                             𝑑𝑑𝑑𝑑
                1
      𝑑𝑑𝑑𝑑 =          𝑑𝑑𝑑𝑑 hence 𝑑𝑑𝑑𝑑 = 2𝑢𝑢 𝑑𝑑𝑑𝑑
               2�𝑓𝑓



Substituting variables:

      � 𝑒𝑒 ln(10)�𝑚𝑚�𝑓𝑓+𝑏𝑏�/10 𝑑𝑑𝑑𝑑 = 2 � 𝑢𝑢 𝑒𝑒 ln(10)(𝑚𝑚𝑚𝑚+𝑏𝑏)/10 𝑑𝑑𝑑𝑑

      Let 𝑣𝑣 = 10
               𝑚𝑚
                  𝑙𝑙𝑙𝑙(10) 𝑢𝑢 and 𝑑𝑑𝑑𝑑 =
                                         𝑚𝑚
                                         10
                                            𝑙𝑙𝑙𝑙(10) 𝑑𝑑𝑑𝑑

                                                                                                                           𝑣𝑣
                  ln(10)(𝑚𝑚𝑚𝑚+𝑏𝑏)/10                            ln(10) 𝑏𝑏/10
                                                                                              𝑣𝑣             ln(10)𝑚𝑚 𝑚𝑚           /10     𝑑𝑑𝑑𝑑
      2 � 𝑢𝑢 𝑒𝑒                                  𝑑𝑑𝑑𝑑 = 2 𝑒𝑒                        � 𝑚𝑚                𝑒𝑒           10 𝑙𝑙𝑙𝑙(10)
                                                                                                                                         𝑚𝑚
                                                                                             𝑙𝑙𝑙𝑙(10)                                       𝑙𝑙𝑙𝑙(10)
                                                                                        10                                               10

                  𝑏𝑏/10
          2�10            �
      =                 2     � 𝑣𝑣 𝑒𝑒 𝑣𝑣 𝑑𝑑𝑑𝑑
           𝑚𝑚
          �10 𝑙𝑙𝑙𝑙(10)�

Integrating ∫ 𝑣𝑣 𝑒𝑒 𝑣𝑣 𝑑𝑑𝑑𝑑 by parts results in

      � 𝑣𝑣 𝑒𝑒 𝑣𝑣 𝑑𝑑𝑑𝑑 = 𝑣𝑣 𝑒𝑒 𝑣𝑣 − � 𝑒𝑒 𝑣𝑣 𝑑𝑑𝑑𝑑 = 𝑒𝑒 𝑣𝑣 (𝑣𝑣 − 1)

Therefore
                                                                 𝑏𝑏
                   �𝑚𝑚�𝑓𝑓+𝑏𝑏�                            2 �1010 �             𝑚𝑚                                                    2 �10�𝑚𝑚�𝑓𝑓+𝑏𝑏�/10 �
                                                                                                 𝑚𝑚                                                             𝑚𝑚
               � 10 10 𝑑𝑑𝑑𝑑 =                                             𝑒𝑒   10 𝑙𝑙𝑙𝑙(10)�𝑓𝑓   �10 𝑙𝑙𝑙𝑙(10)�𝑓𝑓 − 1� =                                         �10 𝑙𝑙𝑙𝑙(10)�𝑓𝑓 − 1�
                                                         𝑚𝑚           2                                                                       𝑚𝑚           2
                                                        �10 𝑙𝑙𝑙𝑙(10)�                                                                        �10 𝑙𝑙𝑙𝑙(10)�




158                                                                                          CableLabs                                                                      06/27/25
