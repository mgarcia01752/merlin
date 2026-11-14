/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtTest
import "../qml/theme"
import "../qml/instruments/qam"
import "../qml/instruments/spectrum"

Pane {
    id: window
    width: 1180; height: 560; visible: true
    Theme { id: theme }
    ThemePalette { targetControl: window; theme: theme }
    ThemePalette { targetControl: window.Window.window; theme: theme }
    ColumnLayout {
        anchors.fill: parent
        ReceiverCellControls {
            id: controls
            theme: theme
            Layout.fillWidth: true
            metadata: ({profiles:[0], orders:[4096], retained:65536, first_bin:356,
                last_bin:7883, first_symbol:256, last_symbol:272})
        }
        SpectrumPlot {
            id: spectrum
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: theme
            showDetails: false
            viewport: ({points:[{x:0,y:0.4,minimum_y:0.42,readout:"Synthetic PSD fixture"},{x:1,y:0.4,minimum_y:0.42,readout:"Synthetic PSD fixture"}],
                start:4000,count:1000,total:8192,
                x_ticks:[{x:0,label:"-2.400"},{x:0.5,label:"10.088"},{x:1,label:"22.575"}],
                y_ticks:[{y:0,label:"-60"},{y:1,label:"-160"}],
                bands:[{first:112/999,last:127/999,label:"PLC"}],
                axis_label:"PSD · dB re 1 normalized unit²/Hz",
                frequency_label:"Baseband frequency offset (MHz)"})
        }
    }
    TestCase {
        name: "ControlPalette"
        when: windowShown
        function linear(value) { return value <= 0.04045 ? value/12.92 : Math.pow((value+0.055)/1.055,2.4) }
        function luminance(color) { return 0.2126*linear(color.r)+0.7152*linear(color.g)+0.0722*linear(color.b) }
        function contrast(first, second) {
            const a=luminance(first), b=luminance(second)
            return (Math.max(a,b)+0.05)/(Math.min(a,b)+0.05)
        }
        function test_dropdown_and_spin_text_contrast_in_both_themes() {
            const names=["receiverQamSignal","receiverQamView","receiverModulation","receiverFirstBin"]
            for (const dark of [true,false]) {
                theme.dark=dark; wait(100)
                for (const name of names) {
                    const control=findChild(controls,name)
                    verify(control !== null)
                    for (const enabled of [true,false]) {
                        control.enabled=enabled;wait(30)
                        const image=grabImage(control.background)
                        const background=image.pixel(5,5)
                        verify(contrast(control.contentItem.color,background)>=4.5,
                            name+" dark="+dark+" enabled="+enabled+" text="+control.contentItem.color+" background="+background)
                    }
                    control.enabled=true
                }
            }
            theme.dark=true
        }
        function test_plc_band_is_visible_and_below_spectrum_trace() {
            theme.dark=true;wait(100)
            const canvas=findChild(spectrum,"saCanvas")
            verify(canvas !== null)
            const plotWidth=canvas.width-canvas.leftMargin-canvas.rightMargin
            const plotHeight=canvas.height-canvas.topMargin-canvas.bottomMargin
            const y=canvas.topMargin+plotHeight*0.7
            const image=grabImage(canvas)
            const inside=image.pixel(Math.round(canvas.leftMargin+plotWidth*(112+127)/2/999),Math.round(y))
            const outside=image.pixel(Math.round(canvas.leftMargin+plotWidth*0.3),Math.round(y))
            verify(inside.r>outside.r && inside.g>outside.g)
        }
    }
}
