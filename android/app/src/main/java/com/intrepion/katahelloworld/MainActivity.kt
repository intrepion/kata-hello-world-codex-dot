package com.intrepion.katahelloworld

import android.app.Activity
import android.os.Bundle
import android.view.Gravity
import android.widget.TextView

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(TextView(this).apply {
            text = getString(R.string.hello_world)
            textSize = 28f
            gravity = Gravity.CENTER
        })
    }
}
