/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package aws.sdk.kotlin.example.reinvent_2021.webserv

import org.koin.core.context.startKoin

fun main() {
    startKoin {
        modules(mainModule)
    }
}
