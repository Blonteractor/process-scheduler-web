pub mod scheduler_grpc {
    tonic::include_proto!("scheduler");
}

use scheduler::Process;
use scheduler_grpc::scheduler_server::Scheduler;
use scheduler_grpc::{Algorithm, GranttNode, ProcessPayload};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use scheduler::algos::{non_preemptive, preemptive};

pub struct SchedulerService;

#[tonic::async_trait]
impl Scheduler for SchedulerService {
    type RunProcessesStream = ReceiverStream<Result<GranttNode, Status>>;

    async fn run_processes(
        &self,
        request: Request<ProcessPayload>,
    ) -> Result<Response<Self::RunProcessesStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let payload = request.into_inner();

        let mut process_queue: Vec<Process> = payload
            .queue
            .iter()
            .enumerate()
            .map(|(i, p)| {
                Process::new(
                    i,
                    p.arrival_time as usize,
                    p.burst_time as usize,
                    p.priority as usize,
                )
            })
            .collect();

        let scheduler_result: Vec<GranttNode> = match payload.algorithm() {
            Algorithm::Fcfs => {
                non_preemptive::fcfs::first_come_first_serve(process_queue.iter_mut())
            }
            Algorithm::Sjf => non_preemptive::sjf::shortest_job_first(process_queue.iter_mut()),
            Algorithm::Priority => {
                non_preemptive::priority::highest_priority_first(process_queue.iter_mut())
            }
            Algorithm::Srtf => {
                preemptive::srtf::shortest_remaining_time_first(process_queue.iter_mut())
            }
            Algorithm::PriorityPreemptive => {
                preemptive::priority::highest_priority_first(process_queue.iter_mut())
            }
            Algorithm::RoundRobin => {
                if let Some(time_quantum) = payload.quantum {
                    preemptive::round_robin::round_robin(
                        process_queue.iter_mut(),
                        time_quantum as usize,
                    )
                } else {
                    return Err(Status::invalid_argument(
                        "Missing arg: Round Robin was chosen but no quantum was provided",
                    ));
                }
            }
        }
        .grantt_chart
        .into_iter()
        .map(|g| GranttNode {
            pid: g.pid as u32,
            start: g.start as u32,
            end: g.end as u32,
        })
        .collect();

        for node in scheduler_result {
            if let Err(e) = tx.send(Ok(node)).await {
                return Err(Status::internal(e.to_string()));
            }
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
