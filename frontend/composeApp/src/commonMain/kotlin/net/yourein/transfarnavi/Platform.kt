package net.yourein.transfarnavi

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform