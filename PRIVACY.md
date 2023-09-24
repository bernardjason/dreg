# Dreg

This is an open source Android app developed by Bernard Jason. The source code is available on GitHub under the MIT license; the app is also available on Google Play.

I hereby state, to the best of my knowledge and belief, that I have not programmed this app to collect any personally identifiable information. 

Explanation of permissions requested in the app

The list of permissions required by the app can be found in the AndroidManifest.xml file:

AndroidManifest.xml (specified Cargo.toml)

```
 <uses-permission android:name="android.permission.WAKE_LOCK" /> 
```

Why it is required
android.permission.WAKE_LOCK Required to stop the screen switching off during game play

If you find any security vulnerability that has been inadvertently caused by me, or have any question regarding how the app protectes your privacy, please post a discussion on GitHub, and I will fix/help you.
