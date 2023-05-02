function handleHttp(request) {
    // iterate through request.headers

    request.headers.forEach((h, v) => {
        console.log(`${h}: ${v}`)
    })

    // iterate through request.params

    request.params.forEach((p, v) => {
        console.log(`${p}: ${v}`)
    })

    return {
        status: 200,
        headers: null,
        body: "hello world",
    }
}

export const handler = { handleHttp }