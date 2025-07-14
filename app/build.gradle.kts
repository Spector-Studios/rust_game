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
        create("releaseSigning") {
            val inCI = System.getenv("CI")?.toBoolean() ?: false

            val storePath: String?
            val storePassword: String?
            val keyAlias: String?
            val keyPassword: String?

            if (inCI) {
                storePath = System.getenv("KEYSTORE_PATH")
                storePassword = System.getenv("KEYSTORE_PASSWORD")
                keyAlias = System.getenv("KEY_ALIAS")
                keyPassword = System.getenv("KEY_PASSWORD")
            } else {
                storePath = project.findProperty("keyStoreFile") as? String
                storePassword = project.findProperty("keyStorePassword") as? String
                keyAlias = project.findProperty("keyAlias") as? String
                keyPassword = project.findProperty("keyPassword") as? String
            }

            if (storePath != null && storePassword != null && keyAlias != null && keyPassword != null) {
                println("✅ Using custom debug signing config")
                storeFile = file(storePath)
                this.storePassword = storePassword
                this.keyAlias = keyAlias
                this.keyPassword = keyPassword
            } else {
                println("⚠️ No signing config found — will use default Android debug keystore")
                // Do NOT set any properties — Android will fall back to default
            }
        }
    }

    buildTypes {
        getByName("debug") {
            // Only assign signing config if it was successfully configured
            if (signingConfigs.findByName("releaseSigning")?.storeFile != null) {
                signingConfig = signingConfigs.getByName("releaseSigning")
            }
        }

        getByName("release") {
            if (signingConfigs.findByName("releaseSigning")?.storeFile != null) {
                signingConfig = signingConfigs.getByName("releaseSigning")
            }
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
    tasks.named("preBuild").configure {
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
