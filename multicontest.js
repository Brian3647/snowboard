const net = require("net");

const responses = [];

function maybepanic(err) {
	if (err) {
		throw err;
	}
}

function repeat(times, func) {
	for (let i = 0; i < times; i++) {
		func();
	}
}

const client = net.createConnection({ port: 3000 }, () => {
	// 'connect' listener.
	console.log("connected to server!");

	// Send first request
	client.write("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n");

	// Wait for a second, then send second request
	repeat(1000, () => {
		if (
			!client.write(
				"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n",
				maybepanic
			)
		) {
			throw "write failed";
		}
	});

	client.write("GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
});

client.on("data", (data) => {
	responses.push(data.toString());
});

client.on("end", () => {
	console.log("disconnected from server");
	console.log(`expected 1001 responses, got ${responses.length} responses`);
});
