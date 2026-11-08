# 7.2.8 - Triggered RxMER Measurements

Source: CM-SP-PHYv4.0-I07-250627
PDF pages: 103-106
Classification: Main body (DOCSIS 4.0 FDX requirements)

<!-- PDF_PAGE: 103 -->
Physical Layer Specification                                                         CM-SP-PHYv4.0-I07-250627


The number of ZBL symbols (M) in the ZBL Period depends on the number of CMs in the FDX band requiring ZBL
symbols and state of those CMs. The length of this ZBL period will be:




Here LZBL_update,i is the number of ZBL symbols needed by the CMi to do a periodic update of the EC coefficients.
The set CMZBL is the set of CMs in the cable plant that require ZBL symbols for EC update training.
Similarly, the parameter LZBL_initial,i defines the number of ZBL symbols needed by CMi to do initial training.
The CMTS can combine initial training of multiple CMs that require ZBL for initial training.
The CMTS can also combine initial training of some CMs with periodic training of other CMs.
The ZBL period will change based on these combinations.
The CMTS will signal the beginning of the ZBL period and the segments of the ZBL period intended for individual
CM as described in [DOCSIS MULPIv4.0].

7.2.7.5    Echo Cancellation Upstream Grants
Echo cancellers in CMs might require grants in upstream channels defined by the RBA, for the purpose of ALI or
ACI initial or periodic training. These features are part of the CM EC T-REQ message. The use of grants in the
upstream channel for ECT is called foreground training.
If a CM requests foreground training with ZBL, the CMTS synchronizes the upstream grants with downstream ZBL
blocks.
It is possible that the CM might find it difficult to train the EC within the cable environment. For example, the CM
might have specified one method of training initially for an RBA. However, during operation the CM may decide
different training parameters are desired. The CM would then send the CMTS a new EC training request. The EC
training protocol defined in [DOCSIS MULPIv4.0] provides flexibility for CMs that want to change some aspects of
EC training while in operation in the FDX band.

7.2.8     Triggered RxMER Measurements
FDX CMs have the ability to measure RxMER over all subcarriers simultaneously, not just over scattered pilots,
continuous pilots, and PLC preamble, as described in section 9.3.6 of [DOCSIS PHYv3.1]. This type of RxMER
measurement is called a Triggered RxMER because it is triggered in response to a specific event. There are three
types of triggered events defined for FDX CMs: OUDP Sounding Triggered Measurements, ECT RxMER Probe-
Triggered Measurements, and Time-Triggered Measurements. The triggers of all of these measurements are set up
using the OPT messaging specified in MULPI.
OUDP Sounding Triggered Measurements are used for measuring interference between CMs for the purposes of
establishing or refining Interference Groups. For OUDP Sounding Triggered Measurements, the CM performing the
measurements is called a Measurer CM and it makes the measurements in response to grants in MAPs to another
CM's OUDP Sounding SID or OUDP Testing SID. See MULPI for details.
ECT RxMER Probe-Triggered Measurements are intended to measure a CM's receive capabilities during worst case
ALI and ACI and are typically used for setting downstream bit-loading after completion of Echo Cancellation
Training. For ECT RxMER Probe-Triggered Measurements, the CM makes the measurements in response to ECT
RxMER Probes allocated to its own ranging SID.
The Time-Triggered Measurement is a generic tool that allows the CMTS to measure a CM's RxMER under
scenarios of the CMTS's choosing. For the Time-Triggered Measurements, the CM starts its measurements when its
downstream time matches a predetermined downstream timestamp value.
Because these triggered measurements use all subcarriers on all symbols and not just scattered pilots, continuous
pilots, and PLC preamble, the CMTS is responsible for providing ZBL on the measured downstream during the
measurement period, except for the PLC subcarriers and NCP subcarriers.



06/27/25                                            CableLabs                                                    103
<!-- PDF_PAGE: 104 -->
CM-SP-PHYv4.0-I07-250627                                    Data-Over-Cable Service Interface Specifications


7.2.8.1   Measuring Period
The structure of an OUDP sounding period is shown in Figure 26.




                                 Figure 26 - Sounding Period for OUDP Method
The structure of a generic triggered measurement period is shown in Figure 27.

                                  Time

                                                                                     Recovery
                                                                                      Period

                        Downstream Initial               Measuring Period                    Downstream
          Subcarriers
                          OFDM     Period                     (M)                              OFDM




                                          Trigger Event

                                  Figure 27 - Generic Triggered Measurement
The Sounding Period for the OUDP sounding method and the Generic Triggered Measurement consist of an Initial
Period, a Measuring Period, and a Recovery Period, as shown in Figure 27. The Initial Period is intended to enable
the CMTS to configure the system for the required measurement and to flush out the contents of the interleaving
buffers. The CMs will be operating as usual during the Initial Period and hence the contents of the interleaving
buffers will get decoded.
For all Triggered Measurements, the beginning of the Measuring Period will be conveyed to the CMs some time
before the commencement of the Measuring Period, as defined by the [DOCSIS MULPIv4.0] specification.
The Measuring Period is defined using a parameter M OFDM symbols with a maximum value of 1024 (see
[DOCSIS MULPIv4.0]).
The length of the Recovery period is defined using a parameter N in terms of OFDM symbols (see [DOCSIS
MULPIv4.0].


104                                                CableLabs                                            06/27/25
<!-- PDF_PAGE: 105 -->
Physical Layer Specification                                                       CM-SP-PHYv4.0-I07-250627


The FDX CMTS MUST begin inserting Zero-Bit-Loaded (ZBL) symbols into the transmit sequence at a point in the
Initial Period, accounting for time interleaving, such that all subcarriers other than NCP and PLC subcarriers in the
transmitted OFDM symbols are zero-bit-loaded BPSK, throughout the Measuring Period.
The FDX CMTS MUST ensure that these ZBL symbols conform to the corresponding definition in section 7.5.5.3
of [DOCSIS PHYv3.1]. Therefore, these have to be inserted before time and frequency interleaving; each ZBL
symbol has to contain two NCPs, namely, a Null NCP and a CRC NCP.
For OUDP Sounding, the FDX CMTS MUST ensure that this measuring period is fully encompassed by OUDP
OFDMA symbols from the Test CM. Furthermore, the FDX CMTS MUST ensure that the Test CM transmits
OUDP frames for a time period Τ before the measuring period and a time period Τ after the measuring period (this Τ
may be defined in symbols). The minimum value of Τ is 50 μs (or one symbol) and the maximum value of Τ is 5 ms
(or 128 symbols) (see [DOCSIS MULPIv4.0]).
For OUDP Sounding, it is expected that the Measurer CMs will save the receiver state variables comprising gain,
timing/frequency offsets and other relevant parameters, shortly before the beginning of upstream OUDP frames, and
restore these shortly after the end of the upstream OUDP frames.
For ECT RxMER Probe Triggered Measurements, the FDX CMTS MUST ensure that ECT RxMER Probes are
allocated to the measuring CM for the duration of the requested measurement. These probes do not need to be
contiguous and can be sent in groups of 6 or more. (See the ECT section of [DOCSIS MULPIv4.0] for further
restrictions.)
For Time-Triggered Measurements, the FDX CMTS ensures that the desired conditions occur throughout the
measurement period.
The FDX CMs MUST use all M symbols of the measuring period to measure subcarrier MER for triggered
measurements. MER is measured on subcarriers which may contain data during some portion of the time due to the
NCP data subcarriers, non-preamble PLC subcarriers, and frequency interleaving. For the instances of data
occurring in a subcarrier in one of the OFDM symbols of the measuring period, a decision-directed MER
measurement is required, which is different than described in section 9.3.6 of [DOCSIS PHYv3.1]. Since the
constellation point is not known a priori for the Measurer CM in this case, unlike for the conditions provided in
section 9.3.6 of [DOCSIS PHYv3.1], the error vector power may be computed as the square of the magnitude from
the closest constellation point in the decision-directed MER measurement.
The FDX CM MUST compute the MER values of PLC subcarriers, with the accuracy achievable from an averaging
period M subcarriers, for MER values greater than 15 dB, when all M OFDM measurer symbols are PLC preamble
symbols. This accuracy requirement and all following PLC subcarrier MER accuracy requirements do not apply
when any of the M symbols in the averaging period are not preamble symbols. Although there is no accuracy
requirement below 15 dB for PLC subcarriers, the FDX CM MUST ensure that the MER accuracy degrades
gracefully between 15 dB and 13 dB.
The FDX CM MUST NOT set the MER of a PLC subcarrier with MER below 13 dB to a value greater than 15 dB.
The FDX CM MUST compute the MER values of non-PLC subcarriers, with the accuracy capable from an
averaging period M subcarriers, for MER values greater than 5 dB, when all of the symbols are known a priori by
the Measurer CM (no data symbols among the M measurer symbols for a given subcarrier). There are no accuracy
requirements for any MER value computed where any OFDM symbol in a subcarrier's M OFDM measurement
symbols is a data symbol.
Although there is no accuracy requirement below 5 dB for non-PLC subcarriers, the FDX CM MUST ensure that
MER accuracy degrades gracefully between 5 dB and 3 dB.
The FDX CM MUST NOT set the MER of a non-PLC subcarrier with MER below 3 dB to a value greater than 5
dB.
The FDX CM MUST report each MER value in dB using an unsigned 8-bit number comprising 2 fractional bits, i.e.,
u6.2.




06/27/25                                           CableLabs                                                   105
<!-- PDF_PAGE: 106 -->
CM-SP-PHYv4.0-I07-250627                                      Data-Over-Cable Service Interface Specifications


7.2.8.2      Triggered RxMER Measurement Procedure
The MER measurement procedure for OUDP sounding is to be as defined in section 9.3.6 of [DOCSIS PHYv3.1]
for OFDM symbols which are known to the CM a priori. For OFDM symbols which contain NCP data or PLC
subcarrier data, the MER is defined as per section 9.3.6 of [DOCSIS PHYv3.1], but with the error vector power for
such symbols defined as the error vector power between the received value and the ideal constellation point closest
to the received value. In calculating MER when data symbols occur, the CM may use alternative methods rather
than relying on the minimum distance error vector power; some alternative methods are provided in the following
paragraph.
MER of non-PLC subcarriers may also be worked out using the above method. As mentioned earlier, there is some
complexity in working out the locations of the NCP subcarriers in the transmitted domain as well as NCP
modulations. Since the NCP subcarriers occur in isolation as a result of interleaving, the MER of a NCP subcarrier
may be replaced by that of an adjacent BPSK subcarrier. One method of further simplifying this process, without
even working out NCP subcarrier locations, consists of treating all non-PLC subcarriers as BPSK in working out the
MER. This will give poor MER estimates for NCP subcarriers because of the incorrect assumption relating to their
modulation. If a median filter is used to smooth out the subsequent MER profile this will automatically replace the
poor MER values of NCP subcarriers with the median of the neighborhood.

7.2.8.3      Upstream Power During Triggered Measurements
During Triggered Measurements taken in one sub-band while the CM is actively transmitting in another sub-band,
the CM uses the power appropriate for normal operation of the transmitting sub-band.

7.3        PHY Sublayer for FDD OFDM
FDD DOCSIS 4.0 capability requires additional functions to be added to the DOCSIS 3.1 requirements. These new
functions are specified in this section. As with previous generations of DOCSIS technologies, DOCSIS 4.0-
compliant devices will be backward-compatible with previous generations of DOCSIS technology. The FDD
upstream allocated spectrum is comprised of only upstream FDD channels between 108 MHz and the chosen
upstream upper band edge and is further subdivided into a number of 96 MHz upstream OFDMA channels as
appropriate for the chosen upstream upper band edge. Additional detail regarding the FDD upstream allocated
spectrum is provided below in Section 7.3.2, its subsections, and Figure 29.
The CMTS assignment of the FDD upstream band plan can be done incrementally over time as a transition strategy,
from existing DOCSIS networks to extended spectrum DOCSIS networks, as DOCSIS 4.0 FDD CMTSs and
modems become available.
For an FDD DOCSIS system, a distributed architecture is assumed. Thus, this portion of the specification refers to
the physical layer functionality of an FDD-capable CMTS as the FDD node. The CMTS is occasionally used to refer
to the total functionality across the MAC layer and the Physical layer.
A DOCSIS 4.0 FDD-compliant node supports upstream and downstream channels in separate areas of the spectrum
as provided by the chosen band plan and the chosen FDD upstream allocated spectrum. A DOCSIS 4.0 FDD-
compliant CMTS allocates FDD channels to cable modems by providing modems access to upstream channels
within the configured FDD upstream allocated spectrum. FDD channels can be bonded with non-FDD channels and
with other FDD channels.
A DOCSIS 4.0-compliant FDD CM supports two modes of operation related to FDD:
      1.    DOCSIS 3.1 Backward Compatibility Mode: When an FDD CM is using a 204 MHz diplex filter and is
            operating with a DOCSIS 3.1 CMTS, and there is negligible energy above 1218 MHz, the FDD CM
            behaves identically to a DOCSIS 3.1-compliant cable modem and adheres to the requirements in section 7
            of [DOCSIS PHYv3.1].
      2.    DOCSIS 4.0 FDD Mode: A DOCSIS 4.0 FDD CM is in this mode when it is on a DOCSIS 4.0 FDD plant
            with 204 MHz split (high-split) or any of the UHS defined splits UHS-300, UHS-396, UHS-492, or UHS-
            684. In a high-split mode, the CM operates with a single TCS, and in any of the UHS modes, the CM
            operates with two TCSs (TCS and TCS_FDD).




106                                                 CableLabs                                           06/27/25
