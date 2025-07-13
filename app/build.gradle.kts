plugins {
    id("com.android.application") version "8.3.0"
}

repositories {
    google()
    mavenCentral()
}

android {
    compileSdk = 30
    ndkVersion = "27.1.12297006"
    defaultConfig {
        applicationId = "dev.spectorstudios.dungeoncrawl"
        namespace = "dev.spectorstudios.dungeoncrawl"
        minSdk = 25
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    signingConfigs {
        // TODO Create a seperate debug signing
        getByName("debug") {
            if (System.getenv()["CI"].toBoolean()) { // CI=true is exported by Codemagic
                storeFile = file(System.getenv("KEYSTORE_PATH") ?: error("Store path"))
                storePassword = System.getenv("KEYSTORE_PASSWORD") ?: error("Store pass")
                keyAlias = System.getenv("KEY_ALIAS") ?: error("Key alias")
                keyPassword = System.getenv("KEY_PASSWORD") ?: error("Key pass")
            } else {
                storeFile = file(project.findProperty("keyStoreFile") as? String ?: error("Store path P"))
                storePassword = project.findProperty("keyStorePassword") as? String ?: error("Store pass P")
                keyAlias = project.findProperty("keyAlias") as? String ?: error("Key alias P")
                keyPassword = project.findProperty("keyPassword") as? String ?: error("Key pass P")
            }
        }

        create("release") {
            if (System.getenv()["CI"].toBoolean()) { // CI=true is exported by Codemagic
                storeFile = file(System.getenv("KEYSTORE_PATH") ?: error("Store path"))
                storePassword = System.getenv("KEYSTORE_PASSWORD") ?: error("Store pass")
                keyAlias = System.getenv("KEY_ALIAS") ?: error("Key alias")
                keyPassword = System.getenv("KEY_PASSWORD") ?: error("Key pass")
            } else {
                storeFile = file(project.findProperty("keyStoreFile") as? String ?: error("Store path P"))
                storePassword = project.findProperty("keyStorePassword") as? String ?: error("Store pass P")
                keyAlias = project.findProperty("keyAlias") as? String ?: error("Key alias P")
                keyPassword = project.findProperty("keyPassword") as? String ?: error("Key pass P")
            }
        }
    }
    
    buildTypes {
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }

    sourceSets {
        getByName("main") {
            assets.srcDirs(project(":rustlib").layout.buildDirectory.dir("assets").get().asFile)
        }
        getByName("debug") {
            jniLibs.srcDirs(project(":rustlib").layout.buildDirectory.dir("debug/jniLibs").get().asFile)
        }
        getByName("release") {
            jniLibs.srcDirs(project(":rustlib").layout.buildDirectory.dir("release/jniLibs").get().asFile)
        }
    }
}

afterEvaluate {
    tasks.named("generateDebugAssets").configure {
        dependsOn(":rustlib:copyAssets")
    }

    tasks.named("generateReleaseAssets").configure {
        dependsOn(":rustlib:copyAssets")
    }
    
    tasks.named("mergeDebugJniLibFolders").configure {
        dependsOn(":rustlib:buildRustLibsDebug")
    }
    
    tasks.named("mergeReleaseJniLibFolders").configure {
        dependsOn(":rustlib:buildRustLibsRelease")
    }
}

dependencies {
    implementation("androidx.appcompat:appcompat:1.2.0")
    implementation("com.google.android.material:material:1.2.0")
    implementation("androidx.constraintlayout:constraintlayout:2.0.4")
    testImplementation("junit:junit:4.13.1")
    androidTestImplementation("androidx.test.ext:junit:1.1.2")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.3.0")
}
