import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar

plugins {
    kotlin("jvm") version "1.5.31"
    kotlin("plugin.serialization") version "1.5.31"
    application
    id("com.github.johnrengelman.shadow") version "7.1.0"
    id("org.jlleitschuh.gradle.ktlint") version "10.2.0"
}

group = "aws.sdk.kotlin.example.reinvent_2021.webserv"
version = "1.0-SNAPSHOT"

repositories {
    mavenLocal()
    mavenCentral()
}

dependencies {
    // Kotlin itself
    implementation(kotlin("stdlib"))

    // AWS SDK
    implementation("aws.sdk.kotlin:dynamodb:0.9.4-beta")
    implementation("aws.sdk.kotlin:s3:0.9.4-beta")

    // Ktor web server
    implementation("io.ktor:ktor-server-core:1.6.4")
    implementation("io.ktor:ktor-server-netty:1.6.4")
    implementation("io.ktor:ktor-serialization:1.6.4")
    implementation("ch.qos.logback:logback-classic:1.2.5")

    // Koin DI
    implementation("io.insert-koin:koin-core:3.1.2")
}

val ktorMainClassName = "aws.sdk.kotlin.example.reinvent_2021.webserv.MainKt"

application {
    mainClass.set(ktorMainClassName)
}

tasks {
    named<ShadowJar>("shadowJar") {
        manifest {
            attributes(mapOf("Main-Class" to ktorMainClassName))
        }
    }
}
