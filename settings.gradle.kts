pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

plugins {
    id("com.autonomousapps.build-health") version "2.19.0"
    id("com.android.application") version "8.3.0" apply false
}

rootProject.name = "my-app"

include(":rustlib")
include(":app")
