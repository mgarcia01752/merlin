# Appendix III - FDD RPD/RMD RF Output Profiles at Interface C

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 152-153
Classification: Informative (explicit)

<!-- PDF_PAGE: 152 -->
CM-SP-PHYv4.0-I07-250627                                                                              Data-Over-Cable Service Interface Specifications


Appendix III                                     FDD RPD/RMD RF Output Profiles at Interface C
                                                 (Informative)
Please refer to Figure 28 which shows reference interfaces on an FDD node. Figure 31 shows an example of an
Interface C output.
The node RF launch amplifier shown in Figure 28 generally both applies tilt or slope to the signal at Interface C and
raises the signal level as that signal is launched (Interface D) onto the hardline coaxial cable network. The total
composite power (TCP) at Interface D is expected to be at least 70 dBmV.
Within the node at Interface C, the RMD/RPD generally outputs either a flat or a stepped signal. The output at
interface C could also be tilted and these cases are not addressed in this Appendix. The Interface C output minimally
is 20 dBmV. The output is referred to as the channel power reference setting. At Interface C, power can be adjusted
per channel, and the largest required change from the channel power reference setting for any channel is -10 dBc.
Levels at Interface C can be adjustable both up and down on a channel by channel basis, within the boundaries of
the Interface D total composite power setting, and the channel definition within this DOCSIS 4.0 specification.
Channel power adjustments result from at least three considerations.
      1.                 Maintaining power levels for legacy CPE (e.g., digital video set-top boxes and early versions of DOCSIS
                         modems), of which modern equipment operates to 1002 MHz but could be lower.
      2.                 Providing as much power as possible for DOCSIS 4.0 modems out to 1794 MHz or as far as the operator
                         chooses to operate the plant.
      3.                 Optimizing total composite power out of the fiber node with a desired power tilt level at Interface D to
                         overcome cable loss, such that the node launch amplifier is not overdriven.
With the above, and possibly other considerations, there are many possibilities of waveforms at both Interface C and
D and the intent of this Appendix is to provide just a few examples.
Figure 32 shows an example transmission profile at interface C, and specifically a change in channel power at 1218
MHz, between OFDM channel 3 and OFDM channel 4. Changes in power level can only occur on channel
boundaries. In this example, the channel power reference setting is 20 dBmV and a single step of -10 dBc is applied
to channels above 1218 MHz.
   dBmV/6 MHz




                20
                                                                                                                                      10 dB Power Step

                                                                32 x D3.0
                                      Legacy Signals
                10                                              SC-QAM            OFDM #1         OFDM #2          OFDM #3
                                  (57 x 6 MHz channels)
                                                                 (6 MHz)
                                                                                                                                    OFDM #4           OFDM #5          OFDM #6

                     5      108                           450               642             834             1026             1218              1410             1602             1794
                                                                                                                                                                 Frequency [MHz]

                           Figure 32 - Example Transmission Profile at Interface C (Single Step-down at 1218 MHz)


The step is to occur on channel boundaries. The frequency location in the spectrum a step occurs will be determined
by product requirements provided by operators.
Figure 33 shows multiple steps applied at Interface C. Note that all steps occur on channel boundaries and that the
lowest channel power is -10 dBc as compared to the channel power reference setting.




152                                                                                    CableLabs                                                                           06/27/25
<!-- PDF_PAGE: 153 -->
Physical Layer Specification                                                                                                                                                   CM-SP-PHYv4.0-I07-250627


                                                                                                                                                      -3 dBc
  dBmV/6 MHz   20                                                                                                                                                           -6 dBc
                                                                                                                                                                                               -9 dBc                -10 dBc
                                                                               32 x D3.0
                                                   Legacy Signals
               10                                                              SC-QAM                 OFDM #1           OFDM #2
                                               (57 x 6 MHz channels)                                                                             OFDM #3
                                                                                (6 MHz)                                                                                 OFDM #4
                                                                                                                                                                                            OFDM #5             OFDM #6

                      5                  108                            450                 642                   834                     1026                   1218                1410                1602                  1794
                                                                                                                                                                                                           Frequency [MHz]

                                                      Figure 33 - Example Transmission Profile at Interface C (Multiple Step-down)


Figure 34 shows a step-up for a channel.
                                                                                                                                                   -5 dBc
      dBmV/6 MHz




                   20
                                                                                                                         -10 dBc                                         -10 dBc


                                                                                32 x D3.0
                                                    Legacy Signals
                   10                                                           SC-QAM                OFDM #1
                                                (57 x 6 MHz channels)                                                                            OFDM #3
                                                                                 (6 MHz)
                                                                                                                        OFDM #2                                         OFDM #4             OFDM #5             OFDM #6

                           5              108                            450                    642               834                     1026                   1218                1410               1602               1794
                                                                                                                                                                                                          Frequency [MHz]

                                                             Figure 34 - Example Transmission Profile at Interface C (Step-up)


Figure 35 contains an example where the step is placed at 1002 MHz, and in this case the OFDM channels at higher
frequencies will end at 1770 MHz (not 1794 MHz).
                   dBmV/6 MHz




                                                                                                                                                 -6 dBc
                                20


                                                      Legacy Signals          32 x D3.0
                                10                     (53 x 6 MHz            SC-QAM             OFDM #1           OFDM #2
                                                        channels)              (6 MHz)                                                    OFDM #3                OFDM #4             OFDM #5            OFDM #6


                                     5          108                    426                618               810                    1002                   1194                1386              1578               1770
                                                                                                                                                                                                        Frequency [MHz]

                                Figure 35 - Example Transmission Profile at Interface C (Single 6-dB Step-down at 1002 MHz)




06/27/25                                                                                                   CableLabs                                                                                                             153
