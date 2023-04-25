export function wrapper<Args extends Array<any>, Return>(name: String, callback: (...args: Args) => Return) {
	return (...args: Args) => {
		console.info(`firebase-js-sys: Calling '${name}' with args:`, args)
		const returned = callback(...args)
		if (returned) {
			console.info(`firebase-js-sys: '${name}' returned:`, returned)
		}
		return returned
	}
}