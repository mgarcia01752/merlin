/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"

// Host data is presentation-only; the native Rust direction adapter owns
// accepted configuration, readiness and all scientific admission.
Item {
    id: workbench
    required property var theme
    required property var host
    signal sectionRequested(string sectionId)
    signal draftRequested(string sectionId)
    signal draftApplyRequested(string sectionId, string token, var values)
    signal draftCancelled(string token)
    signal configurationLoadRequested()
    signal captureLoadRequested(string family)
    signal capturePageRequested(string family, int first)
    signal capturePlotRequested(string family, int first, int count)
    signal resourceGridRequested()
    signal instrumentOpenRequested(string kind)
    signal instrumentRequested(string kind, var selection, var context)
    signal resourceGridViewRequested(var request, var context)
    signal resourceGrantRequested(var grantId, var context)
    readonly property var incomingPresentation: host && host.presentation ? host.presentation : ({})
    readonly property var presentation: presentationOwner.value || ({})
    readonly property bool presentationRejected: presentationOwner.rejected
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentationOwner
        theme: workbench.theme
    }
    property bool presentationReady: false
    function adoptPresentation() {
        return presentationReady && presentationOwner.replace(incomingPresentation)
    }
    onIncomingPresentationChanged: adoptPresentation()
    Component.onCompleted: {
        presentationReady = true
        adoptPresentation()
    }
    Component.onDestruction: {
        presentationReady = false
        if (presentationOwner) {
            presentationOwner.clear()
        }
    }

    // Keep the three stages and control group separate within the shared frame.
    readonly property real minimumDiagramWidth: 1100
    readonly property real minimumDiagramHeight: 700
    readonly property real edgeMargin: 32
    readonly property real blockWidth: 200
    readonly property real mainBlockHeight: 96
    readonly property int mainBlockCount: 3
    readonly property real inputBlockHeight: 84
    readonly property real configurationWidth: 220
    readonly property real configurationTop: 84
    readonly property real configurationHeight: 390
    readonly property real configurationInset: 16
    readonly property real configurationHeaderHeight: 48
    readonly property real configurationRowGap: 22
    readonly property real configurationGap: 84
    readonly property real mainTop: 286
    readonly property real upperInputTop: 116
    readonly property real correctionTop: 470

    // Place CMTS control below its receiver, clear of the waveform path.
    readonly property real controlBaseline: (mainTop + mainBlockHeight + correctionTop) / 2
    readonly property real legendBottomMargin: 24
    readonly property real legendGap: 24
    readonly property real legendLineWidth: 48
    readonly property real legendLineHeight: 18
    readonly property real gridSpacing: 20
    readonly property real gridDotRadius: 1
    readonly property real panelRadius: 12
    readonly property real captionFontSize: 12
    readonly property real groupHeadingFontSize: 16
    readonly property real captionGap: 14
    readonly property real controlCollectorGap: 24
    readonly property var sectionDialogs: ({
            ucd: ucdDialog,
            profiles: profilesDialog,
            grants: grantsDialog,
            transmitter: transmitterDialog,
            channel: channelDialog,
            receiver: receiverDialog,
            source: sourceDialog,
            preEqualization: preEqualizationDialog,
            rxmer: rxmerDialog,
            verification: verificationDialog
        })
    readonly property var navigationBlocks: [ucdBlock, profilesBlock, grantsBlock, sourceBlock, transmitterBlock, preEqualizationBlock, rxmerBlock, channelBlock, receiverBlock, verificationBlock]

    /*
     * Return an authoritative readiness presentation supplied by the host.
     * Missing blocks remain unaccepted; no scientific admission occurs here.
     */
    function blockPresentation(sectionId) {
        const blocks = presentation.blocks || ({});
        return blocks[sectionId] || ({
                status: "attention",
                reason: "Accepted upstream state is unavailable."
            });
    }

    /*
     * Return display strings for one read-only drill-in.
     * Missing observations stay unavailable, rather than becoming zero data.
     */
    function detailPresentation(sectionId) {
        const details = presentation.details || ({});
        return details[sectionId] || ({});
    }

    /*
     * Reveal a keyboard-focused block within the scrollable diagram.
     * Only viewport geometry changes; accepted host state is untouched.
     */
    function revealBlock(block) {
        if (block.x < viewport.contentX + edgeMargin) {
            viewport.contentX = Math.max(0, block.x - edgeMargin);
        } else if (block.x + block.width > viewport.contentX + viewport.width - edgeMargin) {
            viewport.contentX = Math.min(viewport.contentWidth - viewport.width, block.x + block.width + edgeMargin - viewport.width);
        }
        if (block.y < viewport.contentY + edgeMargin) {
            viewport.contentY = Math.max(0, block.y - edgeMargin);
        } else if (block.y + block.height > viewport.contentY + viewport.height - edgeMargin) {
            viewport.contentY = Math.min(viewport.contentHeight - viewport.height, block.y + block.height + edgeMargin - viewport.height);
        }
    }

    /*
     * Emit a presentation navigation request and open its local drill-in.
     * Unknown identifiers do nothing; opening never applies configuration.
     */
    function openSection(sectionId) {
        const popup = sectionDialogs[sectionId];
        if (!popup) {
            return;
        }
        sectionRequested(sectionId);
        return popup.openChecked();
    }

    /** Open passive inspection; only the native adapter can admit capture data. */
    function openCapture(family) {
        if (!captureDialog.validFamily(family)) {
            return false;
        }
        captureDialog.family = family;
        return captureDialog.openChecked();
    }

    /** Open a direction-specific scientific consumer and request its native default. */
    function openInstrument(kind) {
        const popup = kind === "qam" ? qamDialog : kind === "spectrum" ? spectrumDialog : kind === "response" ? responseDialog : kind === "fec" ? fecDialog : null;
        if (!popup || !popup.openChecked()) {
            return false;
        }
        instrumentOpenRequested(kind);
        return true;
    }

    /** Open the configured-allocation instrument and request its native viewport. */
    function openResourceGrid() {
        if (!resourceGridDialog.openChecked()) {
            return false;
        }
        resourceGridRequested();
        return true;
    }

    Rectangle {
        id: canvasFrame
        objectName: "upstreamCanvasFrame"
        anchors.fill: parent
        anchors.leftMargin: workbench.theme.workbenchCanvasSideMargin
        anchors.rightMargin: workbench.theme.workbenchCanvasSideMargin
        anchors.topMargin: workbench.theme.workbenchCanvasTopMargin
        anchors.bottomMargin: workbench.theme.workbenchCanvasBottomMargin
        color: workbench.theme.background
        border.color: workbench.theme.line
        radius: workbench.panelRadius
    }
    Flickable {
        id: viewport
        objectName: "upstreamViewport"
        anchors.fill: canvasFrame
        clip: true
        boundsBehavior: Flickable.StopAtBounds
        contentWidth: Math.max(width, workbench.minimumDiagramWidth)
        contentHeight: Math.max(height, workbench.minimumDiagramHeight)
        ScrollBar.horizontal: ScrollBar {
            objectName: "upstreamHorizontalScrollBar"
        }
        ScrollBar.vertical: ScrollBar {
            objectName: "upstreamVerticalScrollBar"
        }
        Item {
            id: diagram
            objectName: "upstreamDiagram"
            width: viewport.contentWidth
            height: viewport.contentHeight
            readonly property real mainLeft: workbench.edgeMargin + workbench.configurationWidth + workbench.configurationGap
            readonly property real mainGap: (width - mainLeft - workbench.edgeMargin - workbench.blockWidth * workbench.mainBlockCount) / 2
            readonly property real dataAxis: workbench.mainTop + workbench.mainBlockHeight / 2
            readonly property real controlX: workbench.edgeMargin + workbench.configurationWidth + workbench.controlCollectorGap
            readonly property real correctionX: transmitterBlock.x - mainGap / 2
            Canvas {
                id: grid
                anchors.fill: parent
                onWidthChanged: requestPaint()
                onHeightChanged: requestPaint()
                Connections {
                    target: workbench.theme
                    function onDarkChanged() {
                        grid.requestPaint();
                    }
                }
                onPaint: {
                    const context = getContext("2d");
                    context.reset();
                    context.fillStyle = workbench.theme.line;
                    for (let row = workbench.gridSpacing; row < height; row += workbench.gridSpacing) {
                        for (let column = workbench.gridSpacing; column < width; column += workbench.gridSpacing) {
                            context.beginPath();
                            context.arc(column, row, workbench.gridDotRadius, 0, Math.PI * 2);
                            context.fill();
                        }
                    }
                }
            }
            Rectangle {
                id: configurationGroup
                objectName: "upstreamConfigurationGroup"
                x: workbench.edgeMargin
                y: workbench.configurationTop
                width: workbench.configurationWidth
                height: workbench.configurationHeight
                radius: workbench.panelRadius
                color: workbench.theme.panel
                border.color: workbench.theme.line
                Label {
                    anchors.top: parent.top
                    anchors.horizontalCenter: parent.horizontalCenter
                    height: workbench.configurationHeaderHeight
                    verticalAlignment: Text.AlignVCenter
                    text: "CMTS Configuration"
                    color: workbench.theme.text
                    font.pixelSize: workbench.groupHeadingFontSize
                    font.bold: true
                }
            }
            UpstreamConnection {
                objectName: "txChannelData"
                anchors.fill: parent
                theme: workbench.theme
                role: "data"
                fromX: transmitterBlock.x
                fromY: diagram.dataAxis
                toX: channelBlock.x + channelBlock.width
                toY: diagram.dataAxis
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "channelReceiverData"
                anchors.fill: parent
                theme: workbench.theme
                role: "data"
                fromX: channelBlock.x
                fromY: diagram.dataAxis
                toX: receiverBlock.x + receiverBlock.width
                toY: diagram.dataAxis
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "sourceData"
                anchors.fill: parent
                theme: workbench.theme
                role: "data"
                fromX: sourceBlock.x + sourceBlock.width / 2
                fromY: sourceBlock.y + sourceBlock.height
                toX: transmitterBlock.x + transmitterBlock.width / 2
                toY: transmitterBlock.y
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "rxmerMeasurement"
                anchors.fill: parent
                theme: workbench.theme
                role: "measurement"
                fromX: rxmerBlock.x + rxmerBlock.width / 2
                fromY: rxmerBlock.y + rxmerBlock.height
                toX: channelBlock.x + channelBlock.width / 2
                toY: channelBlock.y
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "preEqualizationMeasurement"
                anchors.fill: parent
                theme: workbench.theme
                role: "measurement"
                fromX: diagram.correctionX
                fromY: preEqualizationBlock.y
                toX: diagram.correctionX
                toY: diagram.dataAxis
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "receiverVerification"
                anchors.fill: parent
                theme: workbench.theme
                role: "verification"
                fromX: receiverBlock.x + receiverBlock.width / 2
                fromY: receiverBlock.y
                toX: verificationBlock.x + verificationBlock.width / 2
                toY: verificationBlock.y + verificationBlock.height
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "configurationCollector"
                anchors.fill: parent
                theme: workbench.theme
                role: "control"
                fromX: diagram.controlX
                fromY: ucdBlock.y + ucdBlock.height / 2
                toX: diagram.controlX
                toY: workbench.controlBaseline
                arrowHead: false
                showMarker: false
            }
            UpstreamConnection {
                objectName: "configurationControl"
                anchors.fill: parent
                theme: workbench.theme
                role: "control"
                fromX: diagram.controlX
                fromY: workbench.controlBaseline
                toX: receiverBlock.x + receiverBlock.width / 2
                toY: workbench.controlBaseline
                arrowHead: false
                showMarker: false
            }
            UpstreamConnection {
                objectName: "receiverControl"
                anchors.fill: parent
                theme: workbench.theme
                role: "control"
                fromX: receiverBlock.x + receiverBlock.width / 2
                fromY: workbench.controlBaseline
                toX: receiverBlock.x + receiverBlock.width / 2
                toY: receiverBlock.y + receiverBlock.height
                arrowHead: true
                showMarker: true
            }
            UpstreamConnection {
                objectName: "ucdControl"
                anchors.fill: parent
                theme: workbench.theme
                role: "control"
                fromX: ucdBlock.x + ucdBlock.width
                fromY: ucdBlock.y + ucdBlock.height / 2
                toX: diagram.controlX
                toY: ucdBlock.y + ucdBlock.height / 2
                arrowHead: false
                showMarker: false
            }
            UpstreamConnection {
                objectName: "profilesControl"
                anchors.fill: parent
                theme: workbench.theme
                role: "control"
                fromX: profilesBlock.x + profilesBlock.width
                fromY: profilesBlock.y + profilesBlock.height / 2
                toX: diagram.controlX
                toY: profilesBlock.y + profilesBlock.height / 2
                arrowHead: false
                showMarker: false
            }
            UpstreamConnection {
                objectName: "grantsControl"
                anchors.fill: parent
                theme: workbench.theme
                role: "control"
                fromX: grantsBlock.x + grantsBlock.width
                fromY: grantsBlock.y + grantsBlock.height / 2
                toX: diagram.controlX
                toY: grantsBlock.y + grantsBlock.height / 2
                arrowHead: false
                showMarker: false
            }
            WorkbenchBlock {
                id: ucdBlock
                objectName: "upstreamUcdBlock"
                theme: workbench.theme
                heading: "UCD / OFDMA\nConfiguration"
                blockStatus: workbench.blockPresentation("ucd").status || "attention"
                statusReason: workbench.blockPresentation("ucd").reason || "Accepted upstream state is unavailable."
                x: configurationGroup.x + workbench.configurationInset
                y: configurationGroup.y + workbench.configurationHeaderHeight
                width: configurationGroup.width - workbench.configurationInset * 2
                height: workbench.inputBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("ucd")
                Keys.onReturnPressed: workbench.openSection("ucd")
                Keys.onEnterPressed: workbench.openSection("ucd")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: profilesBlock
                objectName: "upstreamProfilesBlock"
                theme: workbench.theme
                heading: "IUC /\nBurst Profiles"
                blockStatus: workbench.blockPresentation("profiles").status || "attention"
                statusReason: workbench.blockPresentation("profiles").reason || "Accepted upstream state is unavailable."
                x: ucdBlock.x
                y: ucdBlock.y + ucdBlock.height + workbench.configurationRowGap
                width: ucdBlock.width
                height: ucdBlock.height
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("profiles")
                Keys.onReturnPressed: workbench.openSection("profiles")
                Keys.onEnterPressed: workbench.openSection("profiles")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: grantsBlock
                objectName: "upstreamGrantsBlock"
                theme: workbench.theme
                heading: "MAP / Grants"
                blockStatus: workbench.blockPresentation("grants").status || "attention"
                statusReason: workbench.blockPresentation("grants").reason || "Accepted upstream state is unavailable."
                x: ucdBlock.x
                y: profilesBlock.y + profilesBlock.height + workbench.configurationRowGap
                width: ucdBlock.width
                height: ucdBlock.height
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("grants")
                Keys.onReturnPressed: workbench.openSection("grants")
                Keys.onEnterPressed: workbench.openSection("grants")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: transmitterBlock
                objectName: "upstreamTransmitterBlock"
                theme: workbench.theme
                heading: "CM Transmitter"
                blockStatus: workbench.blockPresentation("transmitter").status || "attention"
                statusReason: workbench.blockPresentation("transmitter").reason || "Accepted upstream state is unavailable."
                x: channelBlock.x + channelBlock.width + diagram.mainGap
                y: workbench.mainTop
                width: workbench.blockWidth
                height: workbench.mainBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("transmitter")
                Keys.onReturnPressed: workbench.openSection("transmitter")
                Keys.onEnterPressed: workbench.openSection("transmitter")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: channelBlock
                objectName: "upstreamChannelBlock"
                theme: workbench.theme
                heading: "Channel Simulator"
                blockStatus: workbench.blockPresentation("channel").status || "attention"
                statusReason: workbench.blockPresentation("channel").reason || "Accepted upstream state is unavailable."
                x: receiverBlock.x + receiverBlock.width + diagram.mainGap
                y: workbench.mainTop
                width: workbench.blockWidth
                height: workbench.mainBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("channel")
                Keys.onReturnPressed: workbench.openSection("channel")
                Keys.onEnterPressed: workbench.openSection("channel")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: receiverBlock
                objectName: "upstreamReceiverBlock"
                theme: workbench.theme
                heading: "CMTS Receiver"
                blockStatus: workbench.blockPresentation("receiver").status || "attention"
                statusReason: workbench.blockPresentation("receiver").reason || "Accepted upstream state is unavailable."
                x: diagram.mainLeft
                y: workbench.mainTop
                width: workbench.blockWidth
                height: workbench.mainBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("receiver")
                Keys.onReturnPressed: workbench.openSection("receiver")
                Keys.onEnterPressed: workbench.openSection("receiver")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: sourceBlock
                objectName: "upstreamSourceBlock"
                theme: workbench.theme
                heading: "PRBS-TX"
                blockStatus: workbench.blockPresentation("source").status || "attention"
                statusReason: workbench.blockPresentation("source").reason || "Accepted upstream state is unavailable."
                x: transmitterBlock.x
                y: workbench.upperInputTop
                width: workbench.blockWidth
                height: workbench.inputBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("source")
                Keys.onReturnPressed: workbench.openSection("source")
                Keys.onEnterPressed: workbench.openSection("source")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: rxmerBlock
                objectName: "upstreamRxmerBlock"
                theme: workbench.theme
                heading: "RxMER"
                blockStatus: workbench.blockPresentation("rxmer").status || "attention"
                statusReason: workbench.blockPresentation("rxmer").reason || "Accepted upstream state is unavailable."
                x: channelBlock.x
                y: workbench.upperInputTop
                width: workbench.blockWidth
                height: workbench.inputBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("rxmer")
                Keys.onReturnPressed: workbench.openSection("rxmer")
                Keys.onEnterPressed: workbench.openSection("rxmer")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: verificationBlock
                objectName: "upstreamVerificationBlock"
                theme: workbench.theme
                heading: "PRBS-RX"
                blockStatus: workbench.blockPresentation("verification").status || "attention"
                statusReason: workbench.blockPresentation("verification").reason || "Accepted upstream state is unavailable."
                x: receiverBlock.x
                y: workbench.upperInputTop
                width: workbench.blockWidth
                height: workbench.inputBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("verification")
                Keys.onReturnPressed: workbench.openSection("verification")
                Keys.onEnterPressed: workbench.openSection("verification")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            WorkbenchBlock {
                id: preEqualizationBlock
                objectName: "upstreamPreEqualizationBlock"
                theme: workbench.theme
                heading: "CM Pre-Eq"
                blockStatus: workbench.blockPresentation("preEqualization").status || "attention"
                statusReason: workbench.blockPresentation("preEqualization").reason || "Accepted upstream state is unavailable."
                x: diagram.correctionX - workbench.blockWidth / 2
                y: workbench.correctionTop
                width: workbench.blockWidth
                height: workbench.inputBlockHeight
                focusPolicy: Qt.StrongFocus
                onClicked: workbench.openSection("preEqualization")
                Keys.onReturnPressed: workbench.openSection("preEqualization")
                Keys.onEnterPressed: workbench.openSection("preEqualization")
                onActiveFocusChanged: {
                    if (activeFocus) {
                        workbench.revealBlock(this);
                    }
                }
            }
            Label {
                objectName: "upstreamCorrectionLabel"
                x: diagram.correctionX + workbench.captionGap
                y: (diagram.dataAxis + preEqualizationBlock.y) / 2 - implicitHeight / 2
                text: "C before H"
                color: workbench.theme.txScatteredPilot
                font.pixelSize: workbench.captionFontSize
            }
            Label {
                objectName: "upstreamChannelLabel"
                x: channelBlock.x
                y: channelBlock.y + channelBlock.height + workbench.captionGap
                width: channelBlock.width
                text: "Physical channel H"
                horizontalAlignment: Text.AlignHCenter
                color: workbench.theme.muted
                font.pixelSize: workbench.captionFontSize
            }
            Label {
                objectName: "upstreamReceiverLabel"
                x: receiverBlock.x + receiverBlock.width
                y: diagram.dataAxis + workbench.captionGap
                width: diagram.mainGap
                text: "Received observations: C×H"
                wrapMode: Text.WordWrap
                horizontalAlignment: Text.AlignHCenter
                color: workbench.theme.muted
                font.pixelSize: workbench.captionFontSize
            }
            Label {
                x: diagram.controlX
                y: workbench.controlBaseline + workbench.captionGap
                width: receiverBlock.x + receiverBlock.width / 2 - x
                text: "Configuration / grants"
                horizontalAlignment: Text.AlignHCenter
                color: workbench.theme.txPlc
                font.pixelSize: workbench.captionFontSize
            }
            RowLayout {
                objectName: "upstreamConnectionLegend"
                anchors.bottom: parent.bottom
                anchors.bottomMargin: workbench.legendBottomMargin
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: workbench.legendGap
                Repeater {
                    model: [
                        {
                            role: "data",
                            label: "Waveform / data"
                        },
                        {
                            role: "control",
                            label: "Control"
                        },
                        {
                            role: "measurement",
                            label: "Measurement"
                        },
                        {
                            role: "verification",
                            label: "Verification"
                        }
                    ]
                    RowLayout {
                        id: legendEntry
                        required property var modelData
                        UpstreamConnection {
                            Layout.preferredWidth: workbench.legendLineWidth
                            Layout.preferredHeight: workbench.legendLineHeight
                            theme: workbench.theme
                            role: legendEntry.modelData.role
                            fromX: 0
                            fromY: height / 2
                            toX: width
                            toY: height / 2
                        }
                        Label {
                            text: legendEntry.modelData.label
                            color: workbench.theme.text
                            font.pixelSize: workbench.captionFontSize
                        }
                    }
                }
            }
        }
    }
    Label {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: workbench.edgeMargin
        text: presentationOwner.message
        visible: presentationOwner.rejected
        color: workbench.theme.statusAttention
        wrapMode: Text.WordWrap
    }
    UcdConfigurationDialog {
        id: ucdDialog
        objectName: "upstreamUcdDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("ucd")
        onClosed: ucdBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
        onCaptureRequested: function(family) { workbench.openCapture(family); }
    }
    BurstProfilesDialog {
        id: profilesDialog
        objectName: "upstreamProfilesDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("profiles")
        onClosed: profilesBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
    }
    GrantsDialog {
        id: grantsDialog
        objectName: "upstreamGrantsDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("grants")
        onClosed: grantsBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
    }
    TransmitterDialog {
        id: transmitterDialog
        objectName: "upstreamTransmitterDialog"
        theme: workbench.theme
        presentation: workbench.detailPresentation("transmitter")
        onClosed: transmitterBlock.forceActiveFocus()
        onResourceGridRequested: {
            transmitterDialog.close();
            workbench.openResourceGrid();
        }
        onInstrumentRequested: function(kind) {
            transmitterDialog.close();
            workbench.openInstrument(kind);
        }
        onSourceConfigurationRequested: {
            transmitterDialog.close();
            workbench.openSection("source");
        }
    }
    ChannelDialog {
        id: channelDialog
        objectName: "upstreamChannelDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("channel")
        onClosed: channelBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
        onResponseRequested: workbench.openInstrument("response")
    }
    ReceiverDialog {
        id: receiverDialog
        objectName: "upstreamReceiverDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("receiver")
        onClosed: receiverBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
        onResponseRequested: workbench.openInstrument("response")
        onFecRequested: workbench.openInstrument("fec")
    }
    SourceDialog {
        id: sourceDialog
        objectName: "upstreamSourceDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("source")
        onClosed: sourceBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
    }
    PreEqualizationDialog {
        id: preEqualizationDialog
        objectName: "upstreamPreEqualizationDialog"
        theme: workbench.theme
        host: workbench.host
        presentation: workbench.detailPresentation("preEqualization")
        onClosed: preEqualizationBlock.forceActiveFocus()
        onDraftRequested: function(sectionId) { workbench.draftRequested(sectionId); }
        onDraftApplyRequested: function(sectionId, token, values) { workbench.draftApplyRequested(sectionId, token, values); }
        onDraftCancelled: function(token) { workbench.draftCancelled(token); }
        onConfigurationLoadRequested: workbench.configurationLoadRequested()
        onCaptureRequested: function(family) { workbench.openCapture(family); }
        onResponseRequested: workbench.openInstrument("response")
    }
    RxmerDialog {
        id: rxmerDialog
        objectName: "upstreamRxmerDialog"
        theme: workbench.theme
        presentation: workbench.detailPresentation("rxmer")
        captureBusy: workbench.host && workbench.host.upstreamState ? workbench.host.upstreamState.busy === true || workbench.host.upstreamState.application_busy === true : false
        onCaptureRequested: function(family) { workbench.openCapture(family); }
        onClosed: rxmerBlock.forceActiveFocus()
    }
    UpstreamVerificationDialog {
        id: verificationDialog
        objectName: "upstreamVerificationDialog"
        theme: workbench.theme
        presentation: workbench.detailPresentation("verification")
        captureBusy: workbench.host && workbench.host.upstreamState ? workbench.host.upstreamState.busy === true || workbench.host.upstreamState.application_busy === true : false
        onFecRequested: workbench.openInstrument("fec")
        onClosed: verificationBlock.forceActiveFocus()
    }
    UpstreamCaptureDialog {
        id: captureDialog
        objectName: "upstreamCaptureDialog"
        theme: workbench.theme
        readonly property var captureState: workbench.host && workbench.host.upstreamState ? workbench.host.upstreamState.capture_view : null
        presentation: captureState && captureState.family === family ? captureState.view : ({})
        busy: workbench.host && workbench.host.upstreamState ? workbench.host.upstreamState.busy === true || workbench.host.upstreamState.application_busy === true : false
        onLoadRequested: function(family) { workbench.captureLoadRequested(family); }
        onPageRequested: function(family, first) { workbench.capturePageRequested(family, first); }
        onPlotRequested: function(family, first, count) { workbench.capturePlotRequested(family, first, count); }
        onMeasuredPlanRequested: {
            captureDialog.close();
            workbench.configurationLoadRequested();
        }
    }
    ResourceGridDialog {
        id: resourceGridDialog
        objectName: "upstreamResourceGridDialog"
        theme: workbench.theme
        host: workbench.host
        onViewRequested: function(request, context) { workbench.resourceGridViewRequested(request, context); }
        onGrantRequested: function(grantId, context) { workbench.resourceGrantRequested(grantId, context); }
    }
    UpstreamQamDialog {
        id: qamDialog
        objectName: "upstreamQamDialog"
        theme: workbench.theme
        host: workbench.host
        onInstrumentRequested: function(kind, selection, context) { workbench.instrumentRequested(kind, selection, context); }
    }
    UpstreamSpectrumDialog {
        id: spectrumDialog
        objectName: "upstreamSpectrumDialog"
        theme: workbench.theme
        host: workbench.host
        onInstrumentRequested: function(kind, selection, context) { workbench.instrumentRequested(kind, selection, context); }
    }
    UpstreamResponseDialog {
        id: responseDialog
        objectName: "upstreamResponseDialog"
        theme: workbench.theme
        host: workbench.host
        onInstrumentRequested: function(kind, selection, context) { workbench.instrumentRequested(kind, selection, context); }
    }
    UpstreamFecDialog {
        id: fecDialog
        objectName: "upstreamFecDialog"
        theme: workbench.theme
        host: workbench.host
        onInstrumentRequested: function(kind, selection, context) { workbench.instrumentRequested(kind, selection, context); }
    }
}
