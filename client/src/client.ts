import { createChannel, createClient } from "nice-grpc-web";
import { SchedulerDefinition } from "./proto/scheduler/scheduler";

export const client = createClient(
	SchedulerDefinition,
	createChannel("http://localhost10000")
);
