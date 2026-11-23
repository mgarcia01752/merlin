/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtTest
import "../qml/shared/theme"
import "../qml/downstream/measurements"
Pane {
    id:window;width:1240;height:820;visible:true
    Theme {id:theme}
    QtObject {
        id:backend
        property var lastRequest:null
        property int cancelled:0
        property bool pending:false
        property bool exportAllowed:false
        property bool exportSaved:false
        property int exportSaves:0
        property string exportFormat:""
        property var exportSelection:null
        function designer_export_prepare(document,selection){exportSelection=JSON.parse(selection);exportSaved=false;return JSON.stringify({running:true})}
        function designer_export_poll(){return JSON.stringify(exportSaved ? {running:false,saved:true,path:"/fixture/export.bin",summary:{captures:1,samples:7600,missing:10,rounded:3,clipped:0}} : {running:false,can_export:exportAllowed,can_json:true,error:exportAllowed ? "":"Capture 1: carrier 296: outside PNM range.",summary:{captures:1,samples:7600,missing:10,rounded:3,clipped:0}})}
        function designer_export_save(format){exportFormat=format;exportSaves++;exportSaved=true;return JSON.stringify({running:true})}
        function designer_export_cancel(){}
        // Independent fixture anchors for the public limits report; no production fallback.
        property var limits: ({minimum_mer_db:-30,maximum_mer_db:120,maximum_captures:256,maximum_values:4000000,maximum_value_edits:32,maximum_random_standard_deviation_db:20,maximum_absolute_tilt_db:60,maximum_ripple_peak_to_peak_db:60,maximum_notch_depth_db:60,minimum_echo_distance_metres:0.001,maximum_echo_distance_metres:10000})
        function designer_begin(){return JSON.stringify({limits:limits,imported_count:50,first_hz:640000000,last_hz:827000000,spacing_hz:25000,request:{source:"imported",capture_count:50,baseline_db:48,recipe:{tilt:null,ripple:null,notch:null,edits:[]}}})}
        function designer_preview(document,position,start,count){lastRequest=JSON.parse(document);return JSON.stringify({points:[{x:0,y:0.2,minimum_y:0.2,gap:false}],comparison:[{x:0,y:0.1,minimum_y:0.1,gap:false}],bands:[{first:0.4,last:0.41,label:"PLC"}],start:count ? start:0,count:count || 7481,total:7481,plc_view:{start:4000,count:80},plc_summary:"PLC first–last: 740.000–740.375 MHz · 16 carriers",axis_label:"RxMER (dB)",summary:"Capture "+position,timeline:[]})}
        function designer_stage(document,options){lastRequest=JSON.parse(document);pending=true;return JSON.stringify({running:true})}
        function designer_poll(){pending=false;return JSON.stringify({running:false,cancelled:true})}
        function designer_cancel(){cancelled++}
    }
    RxmerDesignerDialog {id:dialog;theme:theme;backend:backend;parentOptions:({mode:"hold",interval_symbols:"1000",tolerance_seconds:90000})}
    RxmerDesignScope {id:scopeProbe;theme:theme;captureCount:50;maximumCaptures:256;visible:false}
    TestCase {
        name:"RxmerDesigner";when:windowShown
        function cleanup(){dialog.close();findChild(dialog,"designerEffectsScroll").contentItem.contentY=0;wait(100)}
        function revealEffect(item){
            const scroll=findChild(dialog,"designerEffectsScroll"), flick=scroll.contentItem
            const point=item.mapToItem(scroll,0,0)
            flick.contentY=Math.max(0,Math.min(flick.contentHeight-flick.height,flick.contentY+point.y-flick.height/2))
            verify(waitForRendering(dialog.contentItem))
        }
        function test_button_domains_follow_the_public_report(){
            const original=backend.limits
            try {
                backend.limits=Object.assign({},original,{maximum_captures:8,maximum_value_edits:3,maximum_absolute_tilt_db:12,maximum_ripple_peak_to_peak_db:7,maximum_notch_depth_db:9,maximum_random_standard_deviation_db:2})
                dialog.open();tryCompare(dialog,"visible",true)
                compare(findChild(dialog,"designerCount").validator.top,8)
                dialog.valueEdits=[{},{}]
                verify(findChild(dialog,"designerAddValueEdit").enabled)
                dialog.valueEdits=[{},{},{}]
                verify(!findChild(dialog,"designerAddValueEdit").enabled)
                dialog.valueEdits=[]
                findChild(dialog,"designerTilt").checked=true
                findChild(dialog,"designerTiltDb").text="12"
                verify(!dialog.canStepTilt(1));verify(dialog.canStepTilt(-1))
                findChild(dialog,"designerRipple").checked=true
                const ripple=findChild(dialog,"designerRippleDb");ripple.text="7"
                verify(!dialog.canStepRipple(ripple,1));verify(dialog.canStepRipple(ripple,-1))
                findChild(dialog,"designerNotch").checked=true
                const depth=findChild(dialog,"designerNotchDepth");depth.text="9"
                verify(!dialog.canStepNotch(depth,1));verify(dialog.canStepNotch(depth,-1))
                findChild(dialog,"designerSource").currentIndex=1
                findChild(dialog,"designerRandomVariation").checked=true
                findChild(dialog,"designerRandomDeviation").text="2"
                verify(!findChild(dialog,"designerRandomDeviationUp").enabled)
                verify(findChild(dialog,"designerRandomDeviationDown").enabled)
            } finally {
                backend.limits=original
            }
        }
        function test_missing_public_limits_disable_scientific_actions(){
            const original=backend.limits
            try {
                backend.limits=null
                dialog.open();tryCompare(dialog,"visible",true)
                verify(!findChild(dialog,"designerApply").enabled)
                verify(!findChild(dialog,"designerExportPnm").enabled)
                verify(!findChild(dialog,"designerAddValueEdit").enabled)
                findChild(dialog,"designerTilt").checked=true
                verify(!dialog.canStepTilt(1))
            } finally {
                backend.limits=original
            }
        }
        function test_random_variation_only_for_flat_source(){
            dialog.open();tryCompare(dialog,"visible",true)
            const source=findChild(dialog,"designerSource"), toggle=findChild(dialog,"designerRandomVariation")
            const deviation=findChild(dialog,"designerRandomDeviation"), seed=findChild(dialog,"designerRandomSeed")
            verify(!toggle.enabled);verify(!deviation.enabled);compare(backend.lastRequest.recipe.random_variation,null)
            source.currentIndex=1;toggle.checked=true;deviation.text="1.5";seed.text="41";dialog.refresh()
            verify(toggle.enabled);verify(deviation.enabled)
            compare(backend.lastRequest.recipe.random_variation.standard_deviation_db,1.5)
            compare(backend.lastRequest.recipe.random_variation.seed,41)
            source.currentIndex=0;dialog.refresh()
            verify(!toggle.enabled);verify(!deviation.enabled);verify(!seed.enabled)
            compare(backend.lastRequest.recipe.random_variation,null)
            source.currentIndex=1;dialog.refresh()
            compare(backend.lastRequest.recipe.random_variation.seed,41)
        }
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
            const upper=up.mapToItem(null,0,0), lower=down.mapToItem(null,0,0)
            compare(upper.x,lower.x);verify(upper.y+up.height<=lower.y)
            verify(up.width<field.width)
            verify(up.height+down.height+up.parent.spacing<=findChild(dialog,"designerTiltDirection").height)
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
        function test_ripple_steps_preserve_fractional_entry_and_limits(){
            dialog.open();tryCompare(dialog,"visible",true)
            const field=findChild(dialog,"designerRippleDb"), spacing=findChild(dialog,"designerRippleSpacing")
            const up=findChild(dialog,"designerRippleDepthUp"), down=findChild(dialog,"designerRippleDepthDown")
            verify(!up.enabled);dialog.stepRipple(field,1);compare(field.text,"2")
            findChild(dialog,"designerRipple").checked=true
            mouseClick(up);compare(backend.lastRequest.recipe.ripple.peak_to_peak_db,3)
            mouseClick(down);compare(field.text,"2")
            field.text="2.5";field.forceActiveFocus();keyClick(Qt.Key_Up);compare(field.text,"3.5")
            keyClick(Qt.Key_Down);compare(field.text,"2.5")
            field.text="60";verify(!up.enabled);field.text="0";verify(!down.enabled)
            field.text="invalid";verify(!up.enabled);verify(!down.enabled)
            field.text="";verify(!up.enabled);field.text="2"
            const spacingUp=findChild(dialog,"designerRippleSpacingUp"), spacingDown=findChild(dialog,"designerRippleSpacingDown")
            mouseClick(findChild(dialog,"designerZoomPlc"))
            spacing.text="20.5";mouseClick(spacingUp);compare(backend.lastRequest.recipe.ripple.spacing.spacing_hz,21500000)
            mouseClick(spacingDown);compare(spacing.text,"20.5")
            spacing.text="1";verify(!spacingDown.enabled)
            spacing.text="4294";verify(!spacingUp.enabled)
            spacing.text="0";verify(!spacingUp.enabled)
            spacing.text="20";findChild(dialog,"designerRippleDistanceMode").checked=true
            verify(!spacingUp.enabled);dialog.stepRipple(spacing,1);compare(spacing.text,"20")
            compare(dialog.viewStart,4000);compare(dialog.viewCount,80)
        }
        function test_notch_steps_keep_band_edge_bounds_scope_and_zoom(){
            dialog.open();tryCompare(dialog,"visible",true)
            const center=findChild(dialog,"designerNotchCenter"), width=findChild(dialog,"designerNotchWidth")
            const depth=findChild(dialog,"designerNotchDepth"), edge=findChild(dialog,"designerNotchEdge")
            verify(!findChild(dialog,"designerNotchCenterUp").enabled)
            dialog.stepNotch(center,1);compare(center.text,"733.5")
            findChild(dialog,"designerNotch").checked=true
            const scope=findChild(dialog,"designerNotchScope");scope.restore({mode:"range",first:10,last:20})
            mouseClick(findChild(dialog,"designerZoomPlc"))
            const fields=[center,width,depth,edge], names=["Center","Width","Depth","Edge"]
            for(let index=0;index<fields.length;index++){
                const field=fields[index], value=Number(field.text)
                const up=findChild(dialog,"designerNotch"+names[index]+"Up"), down=findChild(dialog,"designerNotch"+names[index]+"Down")
                revealEffect(field)
                mouseClick(up);compare(Number(field.text),value+1)
                mouseClick(down);compare(Number(field.text),value)
                field.forceActiveFocus();keyClick(Qt.Key_Up);compare(Number(field.text),value+1)
                keyClick(Qt.Key_Down);compare(Number(field.text),value)
                field.text="invalid";verify(!up.enabled);verify(!down.enabled)
                field.text="";verify(!up.enabled);verify(!down.enabled);field.text=String(value)
            }
            compare(backend.lastRequest.recipe.notch.scope.first,10)
            compare(dialog.viewStart,4000);compare(dialog.viewCount,80)
            center.text="645";verify(!findChild(dialog,"designerNotchCenterDown").enabled)
            verify(!findChild(dialog,"designerNotchWidthUp").enabled)
            center.text="822";verify(!findChild(dialog,"designerNotchCenterUp").enabled)
            center.text="733.5";edge.text="5";verify(!findChild(dialog,"designerNotchEdgeUp").enabled)
            verify(!findChild(dialog,"designerNotchWidthDown").enabled)
            edge.text="0";verify(!findChild(dialog,"designerNotchEdgeDown").enabled)
            depth.text="60";verify(!findChild(dialog,"designerNotchDepthUp").enabled)
            depth.text="0";verify(!findChild(dialog,"designerNotchDepthDown").enabled)
            depth.text="2.5";dialog.stepNotch(depth,1);compare(backend.lastRequest.recipe.notch.depth_db,3.5)
            findChild(dialog,"designerNotch").checked=false
            dialog.stepNotch(depth,1);compare(depth.text,"3.5")
        }
        function test_notch_mouse_steps_fit_editor_with_large_font(){
            const originalSize=dialog.font.pixelSize
            const scroll=findChild(dialog,"designerEffectsScroll")
            const originalWidth=scroll.Layout.preferredWidth
            try {
                dialog.font.pixelSize=12;scroll.Layout.preferredWidth=600
                dialog.open();tryCompare(dialog,"visible",true)
                verify(waitForRendering(dialog.contentItem))
                compare(findChild(dialog,"designerNotchFields").columns,6)
                dialog.font.pixelSize=18;scroll.Layout.preferredWidth=350
                findChild(dialog,"designerNotch").checked=true
                verify(waitForRendering(dialog.contentItem))
                compare(findChild(dialog,"designerNotchFields").columns,3)
                for (const name of ["Center","Width","Depth","Edge"]) {
                    const field=findChild(dialog,"designerNotch"+name)
                    const up=findChild(dialog,"designerNotch"+name+"Up")
                    const down=findChild(dialog,"designerNotch"+name+"Down")
                    verify(up.enabled);verify(down.enabled)
                    revealEffect(field)
                    const point=up.mapToItem(scroll,0,0)
                    verify(point.x>=0 && point.x+up.width<=scroll.width,name+" arrow is horizontally clipped")
                    verify(point.y>=0 && point.y+up.height<=scroll.height,name+" arrow is vertically clipped")
                    const value=Number(field.text)
                    mouseClick(up);compare(Number(field.text),value+1,name+" mouse increase")
                    mouseClick(down);compare(Number(field.text),value,name+" mouse decrease")
                }
            } finally {
                dialog.font.pixelSize=originalSize
                scroll.Layout.preferredWidth=originalWidth
            }
        }
        function test_json_save_remains_available_outside_binary_range(){
            backend.exportAllowed=false;backend.exportSaves=0
            dialog.open();tryCompare(dialog,"visible",true)
            mouseClick(findChild(dialog,"designerExportPnm"))
            const popup=findChild(dialog,"rxmerPnmExportDialog")
            tryCompare(popup,"canJson",true)
            verify(!findChild(popup,"pnmExportSave").enabled)
            mouseClick(findChild(popup,"jsonExportSave"))
            tryCompare(popup,"savedPath","/fixture/export.bin")
            compare(backend.exportFormat,"json");compare(backend.exportSaves,1)
            popup.close();dialog.close()
        }
        function test_pnm_export_uses_rust_guard_and_keeps_draft(){
            backend.exportAllowed=false;backend.exportSaves=0
            dialog.open();tryCompare(dialog,"visible",true)
            const previous=JSON.stringify(backend.lastRequest)
            mouseClick(findChild(dialog,"designerExportPnm"))
            const popup=findChild(dialog,"rxmerPnmExportDialog")
            tryCompare(popup,"visible",true)
            const save=findChild(popup,"pnmExportSave")
            tryVerify(function(){return popup.message.indexOf("carrier 296")>=0})
            verify(!save.enabled);compare(backend.exportSaves,0)
            verify(findChild(popup,"jsonExportSave").enabled)
            backend.exportAllowed=true
            mouseClick(findChild(popup,"pnmExportClamp"))
            verify(!save.enabled)
            tryCompare(save,"enabled",true)
            compare(backend.exportSelection.clamp,true)
            const scope=findChild(popup,"pnmExportScope")
            scope.currentIndex=2
            findChild(popup,"pnmExportFirst").text="10";findChild(popup,"pnmExportLast").text="20"
            verify(!save.enabled);tryCompare(save,"enabled",true)
            compare(backend.exportSelection.first,10);compare(backend.exportSelection.last,20)
            verify(backend.exportSelection.collection)
            mouseClick(save);tryCompare(popup,"savedPath","/fixture/export.bin")
            compare(backend.exportSaves,1);compare(backend.exportFormat,"pnm")
            compare(JSON.stringify(backend.lastRequest),previous)
            verify(!backend.pending)
            mouseClick(findChild(popup,"pnmExportClose"));tryCompare(popup,"visible",false)
            verify(dialog.visible)
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
