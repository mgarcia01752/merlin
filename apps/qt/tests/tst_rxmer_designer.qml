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
        function designer_preview(document,position,start,count){lastRequest=JSON.parse(document);return JSON.stringify({points:[{x:0,y:0.2,minimum_y:0.2,gap:false}],comparison:[{x:0,y:0.1,minimum_y:0.1,gap:false}],bands:[{first:0.4,last:0.41,label:"PLC"}],start:count ? start:0,count:count || 7481,total:7481,plc_view:{start:4000,count:80},plc_summary:"PLC first–last: 740.000–740.375 MHz · 16 carriers",axis_label:"RxMER (dB)",summary:"Capture "+position,timeline:[]})}
        function designer_stage(document,options){lastRequest=JSON.parse(document);pending=true;return JSON.stringify({running:true})}
        function designer_poll(){pending=false;return JSON.stringify({running:false,cancelled:true})}
        function designer_cancel(){cancelled++}
    }
    RxmerDesignerDialog {id:dialog;theme:theme;backend:backend;parentOptions:({mode:"hold",interval_symbols:"1000",tolerance_seconds:90000})}
    RxmerDesignScope {id:scopeProbe;theme:theme;captureCount:50;visible:false}
    TestCase {
        name:"RxmerDesigner";when:windowShown
        function cleanup(){dialog.close();wait(100)}
        function test_unchecked_tilt_disables_controls_and_retains_draft(){
            dialog.open();tryCompare(dialog,"visible",true)
            const toggle=findChild(dialog,"designerTilt"), field=findChild(dialog,"designerTiltDb")
            const scope=findChild(dialog,"designerTiltScope")
            const up=findChild(dialog,"designerTiltUp"), down=findChild(dialog,"designerTiltDown"), direction=findChild(dialog,"designerTiltDirection")
            verify(!toggle.checked);verify(toggle.enabled)
            verify(!field.enabled);verify(!scope.enabled)
            verify(!up.enabled);verify(!down.enabled);verify(!direction.enabled)
            mouseClick(toggle)
            verify(field.enabled);verify(scope.enabled);verify(up.enabled);verify(down.enabled);verify(direction.enabled)
            field.text="-6";scope.restore({mode:"range",first:10,last:20});dialog.refresh()
            mouseClick(toggle)
            verify(!field.enabled);verify(!scope.enabled)
            verify(!up.enabled);verify(!down.enabled);verify(!direction.enabled)
            compare(backend.lastRequest.recipe.tilt,null)
            dialog.stepTilt(1);compare(field.text,"-6")
            verify(findChild(dialog,"designerRipple").enabled)
            verify(findChild(dialog,"designerNotch").enabled)
            mouseClick(toggle)
            compare(field.text,"-6");verify(scope.enabled)
            compare(backend.lastRequest.recipe.tilt.tilt_db,-6)
            compare(backend.lastRequest.recipe.tilt.scope.first,10)
            compare(backend.lastRequest.recipe.tilt.scope.last,20)
        }
        function test_direction_reverses_signed_tilt_without_changing_magnitude_or_zoom(){
            dialog.open();tryCompare(dialog,"visible",true)
            findChild(dialog,"designerTilt").checked=true
            const field=findChild(dialog,"designerTiltDb"), direction=findChild(dialog,"designerTiltDirection")
            verify(direction)
            mouseClick(findChild(dialog,"designerZoomPlc"))
            field.text="-5";compare(direction.text,"Low ← High");mouseClick(direction);compare(direction.text,"Low → High");compare(field.text,"5");compare(backend.lastRequest.recipe.tilt.tilt_db,5)
            direction.forceActiveFocus();keyClick(Qt.Key_Space);compare(field.text,"-5");compare(direction.text,"Low ← High")
            compare(dialog.viewStart,4000);compare(dialog.viewCount,80)
            field.text="-2.5";mouseClick(direction);compare(field.text,"2.5")
            field.text="60";mouseClick(direction);compare(field.text,"-60")
            field.text="0";verify(!direction.enabled)
            field.text="";verify(!direction.enabled)
            field.text="invalid";verify(!direction.enabled)
            field.text="61";verify(!direction.enabled)
        }
        function test_tilt_steps_keep_text_entry_keyboard_and_limits(){
            dialog.open();tryCompare(dialog,"visible",true)
            const field=findChild(dialog,"designerTiltDb"), up=findChild(dialog,"designerTiltUp"), down=findChild(dialog,"designerTiltDown")
            verify(field);verify(up);verify(down)
            findChild(dialog,"designerTilt").checked=true
            field.text="-5";mouseClick(up);compare(field.text,"-4");compare(backend.lastRequest.recipe.tilt.tilt_db,-4)
            mouseClick(down);compare(field.text,"-5")
            field.text="-5.5";field.forceActiveFocus();keyClick(Qt.Key_Up);compare(field.text,"-4.5")
            keyClick(Qt.Key_Down);compare(field.text,"-5.5")
            field.text="60";verify(!up.enabled);verify(down.enabled)
            field.text="-60";verify(!down.enabled);verify(up.enabled)
            field.text="";verify(!up.enabled);verify(!down.enabled)
            field.text="invalid";verify(!up.enabled);verify(!down.enabled)
            field.text="-7";field.forceActiveFocus();keyClick(Qt.Key_Tab);compare(backend.lastRequest.recipe.tilt.tilt_db,-7)
        }
        function test_zoom_plc_navigation_and_capture_changes_keep_window(){
            dialog.open();tryCompare(dialog,"visible",true)
            mouseClick(findChild(dialog,"designerZoomPlc"))
            compare(dialog.viewStart,4000);compare(dialog.viewport.count,80)
            findChild(dialog,"designerPreviewCapture").value=15;dialog.refresh()
            compare(dialog.viewport.count,80)
            const navigation=findChild(dialog,"chartNavigation");verify(navigation)
            navigation.zoom(0.5,0.5);compare(dialog.viewport.count,40)
            navigation.forceActiveFocus();keyClick(Qt.Key_Home)
            compare(dialog.viewStart,0);compare(dialog.viewCount,0);compare(dialog.viewport.count,7481)
            mouseClick(findChild(dialog,"designerZoomPlc"))
            mouseClick(findChild(dialog,"designerFullSpan"))
            compare(dialog.viewCount,0)
        }
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
