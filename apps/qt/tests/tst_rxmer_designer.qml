/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/theme"
import "../qml/inputs/measurement_input"
Pane {
    id:window;width:1240;height:820;visible:true
    Theme {id:theme}
    QtObject {
        id:backend
        property var lastRequest:null
        property int cancelled:0
        property bool pending:false
        function designer_begin(){return JSON.stringify({imported_count:50,first_hz:640000000,last_hz:827000000,spacing_hz:25000,request:{source:"imported",capture_count:50,baseline_db:48,recipe:{tilt:null,ripple:null,notch:null,edits:[]}}})}
        function designer_preview(document,position){lastRequest=JSON.parse(document);return JSON.stringify({points:[{x:0,y:0.2,minimum_y:0.2,gap:false}],comparison:[{x:0,y:0.1,minimum_y:0.1,gap:false}],bands:[{first:0.4,last:0.41,label:"PLC"}],start:0,count:7481,total:7481,axis_label:"RxMER (dB)",summary:"Capture "+position,timeline:[]})}
        function designer_stage(document,options){lastRequest=JSON.parse(document);pending=true;return JSON.stringify({running:true})}
        function designer_poll(){pending=false;return JSON.stringify({running:false,cancelled:true})}
        function designer_cancel(){cancelled++}
    }
    RxmerDesignerDialog {id:dialog;theme:theme;backend:backend;parentOptions:({mode:"hold",interval_symbols:"1000",tolerance_seconds:90000})}
    RxmerDesignScope {id:scopeProbe;theme:theme;captureCount:50;visible:false}
    TestCase {
        name:"RxmerDesigner";when:windowShown
        function cleanup(){dialog.close();wait(100)}
        function test_restore_switches_between_range_and_all(){
            scopeProbe.restore({mode:"range",first:10,last:20})
            compare(scopeProbe.range,true);compare(scopeProbe.value().first,10);compare(scopeProbe.value().last,20)
            scopeProbe.restore({mode:"all"})
            compare(scopeProbe.range,false);compare(scopeProbe.value().mode,"all")
        }
        function test_recipe_and_capture_scope_are_independent(){
            dialog.open();tryCompare(dialog,"visible",true)
            const notch=findChild(dialog,"designerNotch");verify(notch);notch.checked=true;dialog.refresh()
            compare(backend.lastRequest.recipe.notch.scope.mode,"all")
            const scope=findChild(dialog,"designerNotchScope");verify(scope)
            scope.range=true;scope.first="10";scope.last="20";dialog.refresh()
            compare(backend.lastRequest.recipe.notch.scope.first,10)
            compare(backend.lastRequest.recipe.notch.scope.last,20)
            compare(backend.lastRequest.capture_count,50)
            compare(backend.lastRequest.recipe.notch.center_hz,733500000)
            const slider=findChild(dialog,"designerPreviewCapture");slider.value=15;dialog.refresh()
            compare(dialog.viewport.summary,"Capture 15")
            const apply=findChild(dialog,"designerApply");verify(apply.enabled)
        }
        function test_cancel_does_not_stage_and_new_flat_source_is_explicit(){
            const previous=backend.pending
            dialog.open();tryCompare(dialog,"visible",true)
            const source=findChild(dialog,"designerSource");source.currentIndex=1
            findChild(dialog,"designerCount").text="50";dialog.refresh()
            compare(backend.lastRequest.source,"flat");compare(backend.lastRequest.capture_count,50)
            dialog.reject();compare(backend.pending,previous)
        }
    }
}
