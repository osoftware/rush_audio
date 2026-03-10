package pl.net.orb.rush_synth

import android.content.Context
import androidx.annotation.NonNull
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

class RushSynthPlugin : FlutterPlugin, MethodCallHandler {
    companion object {
        init {
            System.loadLibrary("rush_synth")
        }
    }

    external fun init_android(ctx: Context)

    override fun onAttachedToEngine(
        @NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding,
    ) {
        init_android(flutterPluginBinding.applicationContext)
    }

    override fun onMethodCall(
        @NonNull call: MethodCall,
        @NonNull result: Result,
    ) {
        result.notImplemented()
    }

    override fun onDetachedFromEngine(
        @NonNull binding: FlutterPlugin.FlutterPluginBinding,
    ) {
    }
}