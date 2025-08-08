import org.gradle.api.DefaultTask
import org.gradle.api.file.ConfigurableFileCollection
import org.gradle.api.provider.Property
import org.gradle.api.tasks.*

abstract class BuildRustLibs: DefaultTask() {
    @get:InputFiles
    @get:PathSensitive(PathSensitivity.RELATIVE)
    abstract val sourceFiles: ConfigurableFileCollection

    @get:Internal
    abstract val cargoProjectDir: DirectoryProperty

    @get:Input
    abstract val targetArchs: ListProperty<String>

    @get:Input
    abstract val release: Property<Boolean>

    @get:OutputDirectory
    abstract val outputDir: DirectoryProperty

    @Inject
    abstract fun getExecOperations(): ExecOperations

    @TaskAction
    fun build() {
        val outDir = outputDir.get().asFile
        outDir.mkdirs()

        val buildCommand = buildList {
            addAll(listOf("cargo", "ndk"))

            addAll(targetArchs.get().flatMap { listOf("-t", it) })

            addAll(listOf("-o", outDir.absolutePath))

            add("build")

            if (release.get()) add("--release")
        }
        
        getExecOperations().exec(Action<ExecSpec> {
            environment(
                mapOf(
                    "CARGO_TERM_COLOR" to "always",
                )
            )
            
            workingDir = cargoProjectDir.get().asFile
            commandLine(buildCommand)

            isIgnoreExitValue = false
            // standardOutput = System.out
            // errorOutput = System.err
        })
    }
    
}

tasks.register<BuildRustLibs>("buildRustLibsDebug") {
    group = "rust"
    description = "Build debug shared libraries"

    release.set(false)
    targetArchs.set(listOf("arm64-v8a"))
    
    sourceFiles.setFrom(fileTree(rootProject.layout.projectDirectory) {
        include("**/*.rs", "**/Cargo.toml", "Cargo.lock")
        exclude("**/target/**", "**/build/**")
    })

    outputDir.set(layout.buildDirectory.dir("debug/jniLibs"))
    cargoProjectDir.set(rootProject.layout.projectDirectory)
}

tasks.register<BuildRustLibs>("buildRustLibsRelease") {
    group = "rust"
    description = "Build release shared libraries"

    release.set(true)
    targetArchs.set(
        listOf(
            "arm64-v8a",
            "armeabi-v7a",
            "x86",
            "x86_64"
        )
    )
    
    sourceFiles.setFrom(fileTree(rootProject.layout.projectDirectory) {
        include("**/*.rs", "**/Cargo.toml", "Cargo.lock")
        exclude("**/target/**", "**/build/**")
    })

    outputDir.set(layout.buildDirectory.dir("release/jniLibs"))
    cargoProjectDir.set(rootProject.layout.projectDirectory)
}

tasks.register<Copy>("copyAssets") {
    from(rootProject.layout.projectDirectory.dir("assets"))
    into(layout.buildDirectory.dir("assets"))
}
