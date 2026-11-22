/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/shared/theme"
import "../qml/downstream/instruments/slicer"
Pane {
    id:window;width:1160;height:820;visible:true
    Theme {id:theme}
    ThemePalette {targetControl:window;theme:theme}
    ThemePalette {targetControl:window.Window.window;theme:theme}
    ThemePalette {targetControl:dialog;theme:theme}
    QtObject {
        id:backend
        property bool decoder:false
        property bool empty:false
        function qam_selected(query){return JSON.stringify({runs:[{id:3,available:true}],latest_run_profile:3,profiles:[0],orders:[16],retained:128,first_bin:296,last_bin:7895,first_symbol:256,last_symbol:272,available:!empty})}
        function slicer(query,isDecoder){
            decoder=isDecoder
            if(empty) return JSON.stringify({available:false,rows:[],reason:"No retained data for this profile."})
            const bars=[]
            for(let i=0;i<40;i++) bars.push({low:-100+i*5,high:-95+i*5,count:i===17 || i===22 ? 12 : 2,height:i===17 || i===22 ? 1 : 1/6,percent:1})
            return JSON.stringify({available:true,detail:"Illustrative retained-data fixture · 128 cells · post-descrambling LLRs",llr:{count:128},confidence:{count:128,negative_limit:14,positive_limit:14,full:{bars:bars,limit:100,maximum:12,shown:100},near:{bars:[{low:-1,high:0,count:2,height:1,percent:1.56},{low:0,high:1,count:2,height:1,percent:1.56}],limit:10,maximum:2,shown:4},sequence_limit:100,sequence:[{first:0,last:31,low:-100,high:70},{first:32,last:63,low:-60,high:100},{first:64,last:95,low:-5,high:8},{first:96,last:127,low:-100,high:100}]},rows:[{symbol:256,bin:309,order:16,iq:[0.4,0.3],normalized_iq:[0.4,0.3],label:0,distance_squared:0.007,reference:[0.316,0.316],llrs:[4,-3,2,-1],neighborhood:{center:[0.316,0.316],radius:1.58,references:[[0,0],[0.632,0],[-0.632,0],[0,0.632],[0,-0.632]],polygon:[[-0.316,-0.316],[0.316,-0.316],[0.316,0.316],[-0.316,0.316]]}}]})
        }
    }
    ReceiverSlicerDialog {id:dialog;theme:theme;backend:backend}
    TestCase {
        name:"SoftDiagnostics";when:windowShown
        function test_tabs_bounds_scope_and_empty_state(){
            dialog.open();tryCompare(dialog,"opened",true)
            const tabs=findChild(dialog,"receiverSoftTabs")
            compare(tabs.currentIndex,1);compare(backend.decoder,false)
            const plot=findChild(dialog,"softDiagnosticPlot")
            compare(plot.observations.count,128)
            const near=findChild(dialog,"llrNearZero")
            near.checked=true;compare(plot.histogram.limit,10)
            near.checked=false
            wait(100);grabImage(dialog.contentItem.parent).save("slicer-confidence-dark.png")
            theme.dark=false;wait(100);grabImage(dialog.contentItem.parent).save("slicer-confidence-light.png");theme.dark=true
            tabs.currentIndex=0;compare(plot.mode,0)
            verify(findChild(dialog,"receiverDecisionDetails").text.indexOf("Physical label")>=0)
            wait(100);grabImage(dialog.contentItem.parent).save("slicer-symbol-dark.png")
            tabs.currentIndex=2;tryCompare(backend,"decoder",true)
            verify(findChild(dialog,"decoderRunProfile").visible)
            wait(100);grabImage(dialog.contentItem.parent).save("slicer-decoder-dark.png")
            backend.empty=true;dialog.refresh();compare(dialog.diagnosticState.available,false)
            compare(plot.observations.count,undefined)
            dialog.close()
        }
    }
}
