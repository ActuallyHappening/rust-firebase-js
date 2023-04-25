export function wrapper<Args extends Array<any>, Return>(name: String, callback: (...args: Args) => Return) {
	return (...args: Args) => {
		console.info(`firebase-js-sys: Calling '${name}' with args:`, args)
		return callback(...args)
	}
}