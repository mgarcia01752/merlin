# 7.2.7 - Echo Cancellation at the Cable Modem

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 99-102
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 99 -->
Physical Layer Specification                                                     CM-SP-PHYv4.0-I07-250627


                           Fractional Offset for π/2   Fractional Offset for 2π/3   Fractional Offset for π
     Subcarrier            Phase Rotation Across        Phase Rotation Across       Phase Rotation Across
      Spacing       Ncp     Scattered Pilot Period      Scattered Pilot Period      Scattered Pilot Period
                    512              1/576                        1/432                        1/288
                    768              1/608                        1/456                        1/304
                   1024              1/640                        1/480                        1/320
        25 kHz      192              1/524                        1/393                        1/262
                    256              1/528                        1/396                        1/264
                    512              1/544                        1/408                        1/272
                    768              1/560                        1/420                        1/280
                   1024              1/576                        1/432                        1/288


    •   The FDX CMTS MUST support a configurable variable (Kboost) per sub-band that is used to boost the level
        of sounding CW signals as described in Section 7.2.6.2.4.
Refer to Appendix II for an example of using the MER measurement from the CMs using CWT sounding in the
FDX band to form IGs.
The FDX CMTS MUST ensure the subcarrier index that specifies a CW signal frequency location in the CW-REQ
message matches a DS subcarrier where a coinciding US subcarrier is also defined per UCD message ([DOCSIS
MULPIv4.0]) for OFDMA transmission on the FDX downstream channel under test.
The FDX CM MUST set the center frequency of the CWT signal using the parameters specified in the CW-REQ
message based on the following calculation:
        CWT Signal Center Frequency = CWT Center Frequency of Subcarrier 0 + CW subcarrier Index *
        Subcarrier Spacing + CWT Frequency Offset
NOTE: CW_subcarrier_index is a vector of indices locating the CWT subcarrier frequencies.
The FDX CM MUST set the CWT subcarrier frequencies in accordance with the frequency accuracy requirements
of Section 7.2.4.2.1, Channel Frequency Accuracy. The FDX CM MUST synchronize the frequency accuracy of the
CWT subcarriers with the data subcarriers (e.g., derived from the same Master Clock).

7.2.6.2.6   CM Requirements
    •   The FDX CM MUST transmit the sounding CW signal at the center frequency specified in the sounding
        message.
The FDX CM reports MER values when it participates as a listener in initial sounding opportunities using the same
process defined for RxMER defined in section 9.3.6 of [DOCSIS PHYv3.1].
    •   The FDX CM MUST transmit sounding CW signals at power levels equal to the level of the ranged
        subcarriers at the sounding frequencies plus Kboost for the sub-band.
When the FDX CM receives an OPT-REQ message [DOCSIS MULPIv4.0], it is required to calculate Rx MER as
defined in section 9.3.6 of [DOCSIS PHYv3.1] and return the calculated value. The FDX CMTS applies this method
for determining Rx MER at the FDX CM when the FDX CM participates as a listener in periodic sounding
opportunities.

7.2.7   Echo Cancellation at the Cable Modem
Echo cancellation is used to improve FDX CM receiver performance by cancelling Adjacent Leakage Interference
(ALI) and Adjacent Channel Interference (ACI) resulting from the CM's own upstream transmissions. The CM
referenced in this section and its subsections are FDX CMs.
ALI refers to the power that leaks into a downstream channel of a CM from an upstream transmission of the same
CM in another part of the FDX spectrum. The CM has to transmit at a relatively high-power level to be received by
the FDX node, and as a result the power of the out-of-band components of this upstream transmission are



06/27/25                                          CableLabs                                                  99
<!-- PDF_PAGE: 100 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


comparable to the power of a downstream signal in an adjacent channel at CM input. Some of this upstream out-of-
band power gets coupled into the receiver path through the coupler within the CM, shown in Figure 22. Further out-
of-band power gets added to the received signal through reflections in the drop cable and at the connection with the
main cable. The sum of all these out-of-band components of the upstream transmission that gets added to the
downstream signal is referred to as ALI. This ALI level can be significantly higher than the noise floor of the system
Figure 23 and, therefore, its cancellation is required for the CM to decode data in subcarriers with moderate to high
order QAM constellations.
ACI refers to the power that remains in the same band as the transmitted signal but gets added into the receiver path
through the coupler within the CM as well as through reflections in the cable and its taps. This is significantly
stronger than ALI, but it is not an in-band interference like ALI. Its main effect is in overloading the receiver
circuitry. Hence, although precise cancellation is not needed as in the case of ALI, some cancellation is beneficial to
reduce the load on the receiver analog and analog-to-digital conversion circuitry. It is important to note that the ALI
and ACI referred to above, illustrated in Figure 23, are interferences resulting from upstream transmissions of the
specific receiving CM, and hence these can be categorized as self-ALI and self-ACI, respectively. The reception of
this CM can also be impacted by ALI and ACI from upstream transmissions of other CMs in the cable plant, in
particular, other CMs in the same IG. The CM is required only to cancel its own ALI and ACI, that is, self-ALI and
self-ACI, sufficiently to meet the performance requirement defined in Section 7.2.5.12.3.




                               Figure 22 - Cable Modem and Drop Cable Schematic
Only the first tap is shown in Figure 22 because reflections from beyond this tap are too small to impact CM
performance.




                             Figure 23 - ALI and ACI Illustrated in the Spectral Plane




100                                                 CableLabs                                              06/27/25
<!-- PDF_PAGE: 101 -->
Physical Layer Specification                                                                       CM-SP-PHYv4.0-I07-250627


Echo cancellation can only be carried out after Interference Group (IG) discovery has been completed and the CM
has been assigned to a Transmission Group (TG). All the CMs in a TG have the same Resource Block Assignment
(RBA), that is, the definition of upstream/downstream for each of the FDX channels. Hence after TG assignment the
CM knows its upstream and downstream channels, and it is in a position to commence the training of the echo
cancellers associated with each of the downstream channels.
The sequence of operations in the process of entry of a CM into the FDX band is illustrated in Figure 24.




               CM               Ranging &         FDX DS                                                   Steady State
                                                                                  Echo Canceller
          Registration in       Probing on        channel        IG Discovery                                 FDX
                                                                                  Initial Training
            non-FDX             all FDX US       acquisition                                                Operation




          Frequency & Timing acquired.
          CM Operational on legacy channels.


                            All FDX US ranged and probed.

                                               All FDX DS added to CM.

                                                                    CM assigned to IG
                                                                    CM requests EC Training

                                                                                              CM's EC sufficiently converged




                                                Figure 24 - CM FDX Entry Sequence
An FDX CM that wants to join the FDX network is to begin by first registering with CMTS using the legacy band.
The CM becomes operational on legacy channels. At some point after this, the CMTS adds FDX channels to the
CM, performs IG Discovery for that CM, and eventually assigns a TG-ID to the CM.
At this point, the CM knows which sub-bands are upstream and downstream, and hence, it is in a position to train
the echo cancellers needed for receiving the downstream channels. However, before training can commence an
exchange of information between CM and CMTS is needed to identify the capabilities of the Echo Canceller (EC)
within the CM. EC capabilities define what the CMTS need to provide in order to enable the CM to do echo
canceller training. These include any special OFDM symbols that the CMTS has to transmit on downstream
channels and any specific grants that the CMTS has to provide on upstream channels.
Initial training will work out the impulse or frequency responses that the CM needs for echo cancellation. Once
initial EC training is complete, the CM can transmit user data upstream in its allotted upstream channels without
significantly impacting its FDX downstream channels.
Control of Echo Cancellation Training is detailed in [DOCSIS MULPIv4.0].

7.2.7.1      Echo Cancellation Training Stages
There are two stages of echo canceller training:
    1.     Initial training
    2.     Periodic training




06/27/25                                                       CableLabs                                                      101
<!-- PDF_PAGE: 102 -->
CM-SP-PHYv4.0-I07-250627                                       Data-Over-Cable Service Interface Specifications


Echo Cancellation Training is performed on a per-RBA basis. Initial training occurs when a CM joins the FDX
band. It involves acquiring the impulse or frequency response, of the upstream channel as seen by the CM, in order
to facilitate echo cancellation.
Periodic training is needed at regular intervals to account for changes in the response of the channel mentioned
above. The periodicity of these updates, namely, how frequently these are done, depends on the type of cable plant
as well CM implementation. The EC training protocol can be designed to adapt this update frequency based on the
performance characteristics of cable modems. For example, if performance of the CMs does not deteriorate between
successive updates, the Periodic Training interval can be increased.
In the above description, it has been assumed that after initial training the EC of a CM enters a tracking state for that
RBA. However, the EC can lose convergence for a variety of reasons, for example, burst noise during training.
Therefore, a CM will be able to identify that its EC has lost convergence and report this to the CMTS. In return, the
FDX CMTS provides whatever training opportunities the FDX CM requests to enable the CM to return to operating
within the FDX band.

7.2.7.2    Echo Cancellation Training Status
The CM will incorporate features to identify that its EC has converged. If the convergence is unsatisfactory then the
CMTS might have to provide further training opportunities to the affected CM, for example, by sending further ZBL
blocks. The protocol allows the CM to indicate what types of additional training opportunities it needs.
The CM will report its EC training status to the CMTS when needed, as described in [DOCSIS MULPIv4.0].

7.2.7.3    Echo Cancellation Algorithms
Echo Cancellation algorithms are proprietary to CM design and hence will not form part of this specification.
Furthermore, no specific echo cancellation algorithms are mandated by this specification.
This specification only covers the features that the CMTS has to provide to facilitate the application of these echo
cancellation algorithms.

7.2.7.4    Zero-Bit-Loaded Blocks in Downstream Channels
For initial and periodic training, an FDX CM might require zero-bit-loaded OFDM symbols. If one or more FDX
CMs requiring ZBL for update training exist in the FDX band, or if an FDX CM requiring ZBL for initial training is
awaiting EC initial training, the FDX CMTS MUST include an appropriate ZBL Block in the corresponding
downstream channel.
A ZBL Block is a block of zero-bit-loaded (ZBL) symbols inserted by the FDX CMTS into a downstream OFDM
channel. The FDX CMTS MUST ensure that these ZBL symbols conform to the definition of such symbols given in
section 7.5.5.3 of [DOCSIS PHYv3.1], "Randomization," since there might be FDX-L CMs receiving this
downstream. Therefore, these ZBL symbols are to be created prior to time and frequency interleaving by inserting
BPSK subcarriers instead of data subcarriers. Each such symbol will have two NCPs, a Null NCP and a CRC NCP.
All pilots and PLC will remain as in a data carrying OFDM symbol. A ZBL block will take the structure shown in
Figure 25.




                                    Figure 25 - ZBL Block for Echo Cancellation
The initial period is intended to configure the downstream transmission for inserting ZBL symbols and for flushing
out the interleavers and network jitter smoothing buffers. The ZBL period in Figure 25 is the period in which all
active subcarriers of the OFDM symbols, except the PLC and NCP subcarriers, are BPSK modulated. If the
convolutional time-interleaving depth is I there will be (I-1) symbols with partially ZBL symbols in the initial period
and also in the final period.


102                                                  CableLabs                                               06/27/25
