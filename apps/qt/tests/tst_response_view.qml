/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "fixtures"
import "../qml/shared/theme"
import "../qml/downstream/instruments/response"
Pane {
    id:window;width:1160;height:760;visible:true
    PresentationBudget {
        id: presentationBudget
        // Independent capacities for existing presentation-only fixture graphs.
        maximumBytes: 10000000
        maximumEntries: 100000
        maximumViews: 96
        maximumOpen: 12
    }
    Theme { presentationBackendFactory: presentationBudget.createAdapter;id:theme}
    ThemePalette {targetControl:window;theme:theme}
    ThemePalette {targetControl:window.Window.window;theme:theme}
    ThemePalette {targetControl:dialog;theme:theme}
    QtObject {
        id:backend
        function response(document) {
            const query=JSON.parse(document)
            const points=[]
            for(let i=0;i<=100;i++) points.push({x:i/100,y:0.5+0.2*Math.sin(i/12),minimum_y:0.5+0.2*Math.sin(i/12),gap:i===0,readout:"Illustrative gain"})
            const plot={available:true,points:points,x_ticks:[{x:0,label:"640"},{x:1,label:"832"}],y_ticks:[{y:0,label:"3"},{y:1,label:"-3"}],axis_label:"Gain · dB",frequency_label:"Frequency (MHz)",bands:query.view===4 ? [] : [{first:0.06,last:0.08,label:"PLC"}]}
            return JSON.stringify(Object.assign({},plot,{comparison:plot,mce:{tracking:true,noise_text:"1e-4"},snapshots:[256],summary:"Illustrative display fixture · PLC region from first to last carrier"}))
        }
    }
    ReceiverResponseDialog {id:dialog;theme:theme;backend:backend;sourceIndex:2}
    TestCase {
        name:"ResponseView";when:windowShown
        function test_frequency_and_echo_controls() {
            dialog.openChecked();tryCompare(dialog,"opened",true)
            findChild(dialog,"receiverResponseSource").currentIndex=2
            const velocity=findChild(dialog,"receiverVelocity")
            compare(velocity.visible,false)
            compare(dialog.response.bands.length,1)
            compare(dialog.response.comparison.bands.length,1)
            wait(100);grabImage(dialog.contentItem.parent).save("response-plc-dark.png")
            theme.dark=false;wait(100);grabImage(dialog.contentItem.parent).save("response-plc-light.png")
            dialog.viewIndex=4;dialog.refresh()
            compare(velocity.visible,true)
            compare(dialog.response.bands.length,0)
            dialog.close()
        }
    }
}
