import { createChannel, createClient } from "nice-grpc-web";
import { Algorithm, SchedulerDefinition } from "./proto/scheduler/scheduler";

const client = createClient(
	SchedulerDefinition,
	createChannel("http://localhost:10000")
);

async function main() {
	client.echo({ text: "GEEGEE" }).then((response) => {
		console.log(response);
	});

	const stream = client.runProcesses({
		queue: [
			{
				id: 0,
				arrivalTime: 0,
				burstTime: 4,
				priority: 0,
			},
			{
				id: 1,
				arrivalTime: 2,
				burstTime: 3,
				priority: 0,
			},
		],
		algorithm: Algorithm.FCFS,
	});

	for await (const node of stream) {
		console.log(`${node.pid}:  ${node.start} -> ${node.end}`);
	}
}

main().catch((error) => {
	console.error(error);
	process.exit(1);
});
