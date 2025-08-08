plugins {
    id("com.android.application") version "8.3.0"
}

repositories {
    google()
    mavenCentral()
}

java {
    toolchain.languageVersion.set(JavaLanguageVersion.of(21)) // or 11
}

android {
    compileSdk = 30
    ndkVersion = "27.1.12297006"
    defaultConfig {
        applicationId = "dev.spectorstudios.dungeoncrawl"
        namespace = "dev.spectorstudios.dungeoncrawl"
        minSdk = 28
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
                println("✅ Using custom signing config")
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
            assets.srcDirs(project(":rust-android-build").layout.buildDirectory.dir("assets").get().asFile)
        }
        getByName("debug") {
            jniLibs.srcDirs(project(":rust-android-build").layout.buildDirectory.dir("debug/jniLibs").get().asFile)
        }
        getByName("release") {
            jniLibs.srcDirs(project(":rust-android-build").layout.buildDirectory.dir("release/jniLibs").get().asFile)
        }
    }
}

tasks.withType<JavaCompile>().configureEach {
    options.compilerArgs.add("-Xlint:deprecation")
}

afterEvaluate {
    listOf(
        "preBuild",
        "explodeAssetSourceRelease"
    ).forEach {taskName ->
        tasks.named(taskName).configure {
            dependsOn(":rust-android-build:copyAssets")
        }
    }
    
    tasks.named("mergeDebugJniLibFolders").configure {
        dependsOn(":rust-android-build:buildRustLibsDebug")
    }
    
    tasks.named("mergeReleaseJniLibFolders").configure {
        dependsOn(":rust-android-build:buildRustLibsRelease")
    }
}

dependencies {
    implementation("androidx.appcompat:appcompat:1.2.0")
    testImplementation("junit:junit:4.13.1")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test:monitor:1.6.1")
    androidTestImplementation("junit:junit:4.13.2")
}
