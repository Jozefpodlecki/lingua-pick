
1. Proper architecture

At the start of the app we initialize necessary things

app needs to check local storage whether selected language exist, if not we have to reroute to home page

LanguagePicker should be able to distingush whether we currently have an active language and give different propmt than
"What would you like to learn?

2. components shouldnt do a lot of logic aside from render, that's why we have context, providers reductible etc
3. tauri setup should be in separate module lib.rs should be slim