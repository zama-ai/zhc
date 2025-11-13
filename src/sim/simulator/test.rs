use serde::Serialize;

use super::*;

pub static FREQ: MHz = MHz(400_000_000.);

// Simple counter that increments until reaching a limit
#[derive(Debug, Default, Serialize)]
struct Counter {
    value: usize,
    target: usize,
}

#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq)]
enum CounterEvent {
    #[default]
    Increment,
}

impl std::fmt::Display for CounterEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CounterEvent::Increment => write!(f, "Increment"),
        }
    }
}

impl Event for CounterEvent {}

impl Counter {
    fn new(target: usize) -> Self {
        Self { value: 0, target }
    }
}

impl Simulatable for Counter {
    type Event = CounterEvent;

    fn handle(
        &mut self,
        dispatcher: &mut Dispatcher<Self::Event>,
        trigger: Trigger<Self::Event>,
    ){
        match trigger.event {
            CounterEvent::Increment => {
                self.value += 1;
                if self.value < self.target {
                    // Schedule next increment after 2 cycles
                    dispatcher.dispatch_later(Cycle(2), CounterEvent::Increment)
                }
            }
        }
    }
}

// Ping-pong system that alternates between two events
#[derive(Debug, Default, Serialize)]
struct PingPong {
    ping_count: usize,
    pong_count: usize,
    max_rounds: usize,
}

#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq)]
enum PingPongEvent {
    #[default]
    Ping,
    Pong,
}

impl std::fmt::Display for PingPongEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PingPongEvent::Ping => write!(f, "Ping"),
            PingPongEvent::Pong => write!(f, "Pong"),
        }
    }
}

impl Event for PingPongEvent {}

impl PingPong {
    fn new(max_rounds: usize) -> Self {
        Self {
            ping_count: 0,
            pong_count: 0,
            max_rounds,
        }
    }
}

impl Simulatable for PingPong {
    type Event = PingPongEvent;

    fn handle(
        &mut self,
        dispatcher: &mut Dispatcher<Self::Event>,
        trigger: Trigger<Self::Event>,
    ){
        match trigger.event {
            PingPongEvent::Ping => {
                self.ping_count += 1;
                if self.ping_count <= self.max_rounds {
                    // Pong responds after 3 cycles
                    dispatcher.dispatch_later(Cycle(3), PingPongEvent::Pong);
                }
            }
            PingPongEvent::Pong => {
                self.pong_count += 1;
                if self.pong_count < self.max_rounds {
                    // Ping responds after 5 cycles
                    dispatcher.dispatch_later(Cycle(5), PingPongEvent::Ping);
                }
            }
        }
    }
}

// Timer that generates periodic events
#[derive(Debug, Default, Serialize)]
struct Timer {
    ticks: usize,
    max_ticks: usize,
    interval: Cycle,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
enum TimerEvent {
    Tick,
}

impl std::fmt::Display for TimerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerEvent::Tick => write!(f, "Tick"),
        }
    }
}

impl Event for TimerEvent {}

impl Timer {
    fn new(max_ticks: usize, interval: Cycle) -> Self {
        Self {
            ticks: 0,
            max_ticks,
            interval,
        }
    }
}

impl Simulatable for Timer {
    type Event = TimerEvent;

    fn handle(
        &mut self,
        dispatcher: &mut Dispatcher<Self::Event>,
        trigger: Trigger<Self::Event>,
    ){
        match trigger.event {
            TimerEvent::Tick => {
                self.ticks += 1;
                if self.ticks < self.max_ticks {
                    dispatcher.dispatch_later(self.interval, TimerEvent::Tick)
                }
            }
        }
    }
}

// Pipeline with multiple stages
#[derive(Debug, Default, Serialize)]
struct Pipeline {
    stage1_count: usize,
    stage2_count: usize,
    stage3_count: usize,
    items_to_process: usize,
    items_started: usize,
}

#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq)]
enum PipelineEvent {
    #[default]
    StartItem,
    Stage1Complete,
    Stage2Complete,
    Stage3Complete,
}

impl std::fmt::Display for PipelineEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineEvent::StartItem => write!(f, "StartItem"),
            PipelineEvent::Stage1Complete => write!(f, "Stage1Complete"),
            PipelineEvent::Stage2Complete => write!(f, "Stage2Complete"),
            PipelineEvent::Stage3Complete => write!(f, "Stage3Complete"),
        }
    }
}

impl Event for PipelineEvent {}

impl Pipeline {
    fn new(items_to_process: usize) -> Self {
        Self {
            stage1_count: 0,
            stage2_count: 0,
            stage3_count: 0,
            items_to_process,
            items_started: 0,
        }
    }
}

impl Simulatable for Pipeline {
    type Event = PipelineEvent;

    fn handle(
        &mut self,
        dispatcher: &mut Dispatcher<Self::Event>,
        trigger: Trigger<Self::Event>,
    ){
        match trigger.event {
            PipelineEvent::StartItem => {
                self.items_started += 1;
                // Stage 1 takes 4 cycles
                dispatcher.dispatch_later(Cycle(4), PipelineEvent::Stage1Complete);

                // Start next item if available (with 1 cycle delay)
                if self.items_started < self.items_to_process {
                    dispatcher.dispatch_later(Cycle(1), PipelineEvent::StartItem);
                }
            }
            PipelineEvent::Stage1Complete => {
                self.stage1_count += 1;
                // Stage 2 takes 3 cycles
                dispatcher.dispatch_later(Cycle(3), PipelineEvent::Stage2Complete);
            }
            PipelineEvent::Stage2Complete => {
                self.stage2_count += 1;
                // Stage 3 takes 2 cycles
                dispatcher.dispatch_later(Cycle(2), PipelineEvent::Stage3Complete);
            }
            PipelineEvent::Stage3Complete => {
                self.stage3_count += 1;
                // Pipeline complete for this item
            }
        }
    }
}

#[test]
fn test_empty_simulation() {
    let mut sim: Simulator<Counter> = Simulator::new(FREQ);

    matches!(sim.step(), SimulationState::SimulationOver);
}

#[test]
fn test_simple_counter() {
    let mut sim: Simulator<Counter> = Simulator::new(FREQ);
    sim.simulatable = Counter::new(5);

    // Start the counter
    sim.dispatch_later(Cycle(1), CounterEvent::Increment);

    // Run simulation
    sim.play();

    assert_eq!(sim.simulatable.value, 5);
    assert_eq!(sim.now(), Cycle(9)); // 1 + 2*4 = 9 cycles
}

#[test]
fn test_ping_pong() {
    let mut sim: Simulator<PingPong> = Simulator::new(FREQ);
    sim.simulatable = PingPong::new(3);

    // Start with a ping
    sim.dispatch_later(Cycle(1), PingPongEvent::Ping);

    sim.play();

    assert_eq!(sim.simulatable.ping_count, 3);
    assert_eq!(sim.simulatable.pong_count, 3);
    // Timing: 1(ping) + 3(pong) + 5(ping) + 3(pong) + 5(ping) + 3(pong) = 20
    assert_eq!(sim.now(), Cycle(20));
}

#[test]
fn test_timer() {
    let mut sim: Simulator<Timer> = Simulator::new(FREQ);
    sim.simulatable = Timer::new(4, Cycle(10)); // 4 ticks, every 10 cycles

    // Start timer
    sim.dispatch_later(Cycle(5), TimerEvent::Tick);

    sim.play();

    assert_eq!(sim.simulatable.ticks, 4);
    assert_eq!(sim.now(), Cycle(35)); // 5 + 10 + 10 + 10 = 35
}

#[test]
fn test_pipeline() {
    let mut sim: Simulator<Pipeline> = Simulator::new(FREQ);
    sim.simulatable = Pipeline::new(2); // Process 2 items

    // Start pipeline
    sim.dispatch_later(Cycle(1), PipelineEvent::StartItem);

    sim.play();

    assert_eq!(sim.simulatable.items_started, 2);
    assert_eq!(sim.simulatable.stage1_count, 2);
    assert_eq!(sim.simulatable.stage2_count, 2);
    assert_eq!(sim.simulatable.stage3_count, 2);

    // Item 1: starts at 1, stage1 at 5, stage2 at 8, stage3 at 10
    // Item 2: starts at 2, stage1 at 6, stage2 at 9, stage3 at 11
    assert_eq!(sim.now(), Cycle(11));
}

#[test]
fn test_simultaneous_events() {
    let mut sim: Simulator<Timer> = Simulator::new(FREQ);
    sim.simulatable = Timer::new(10, Cycle(5));

    // Submit multiple events at same time
    sim.dispatch_later(Cycle(1), TimerEvent::Tick);
    sim.dispatch_later(Cycle(1), TimerEvent::Tick);

    sim.play();

    assert_eq!(sim.simulatable.ticks, 11);
}

#[test]
fn test_simulation_step_by_step() {
    let mut sim: Simulator<Counter> = Simulator::new(FREQ);
    sim.simulatable = Counter::new(3);

    sim.dispatch_later(Cycle(2), CounterEvent::Increment);

    // Step 1: process first increment at cycle 2
    let state = sim.step();
    assert_eq!(sim.now(), Cycle(2));
    assert_eq!(sim.simulatable.value, 1);
    assert!(matches!(state, SimulationState::MayContinue));

    // Step 2: process second increment at cycle 4
    let state = sim.step();
    assert_eq!(sim.now(), Cycle(4));
    assert_eq!(sim.simulatable.value, 2);
    assert!(matches!(state, SimulationState::MayContinue));

    // Step 3: process third increment at cycle 6
    let state = sim.step();
    assert_eq!(sim.now(), Cycle(6));
    assert_eq!(sim.simulatable.value, 3);
    assert!(matches!(state, SimulationState::SimulationOver));
}

#[test]
fn test_power_up_scheduling() {
    // Component that schedules initial events via power_up
    #[derive(Default, Debug, Serialize)]
    struct AutoStart {
        boots: usize,
        ticks: usize,
    }

    #[derive(Debug, Clone, Copy, Serialize, PartialEq)]
    enum AutoStartEvent {
        Boot,
        Tick,
    }

    impl std::fmt::Display for AutoStartEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AutoStartEvent::Boot => write!(f, "Boot"),
                AutoStartEvent::Tick => write!(f, "Tick"),
            }
        }
    }

    impl Event for AutoStartEvent {}

    impl Simulatable for AutoStart {
        type Event = AutoStartEvent;

        fn handle(
            &mut self,
            dispatcher: &mut Dispatcher<Self::Event>,
            trigger: Trigger<Self::Event>,
        ){
            match trigger.event {
                AutoStartEvent::Boot => {
                    self.boots += 1;
                    // Schedule first tick after boot
                    dispatcher.dispatch_later(Cycle(5), AutoStartEvent::Tick);
                }
                AutoStartEvent::Tick => {
                    self.ticks += 1;
                    if self.ticks < 3 {
                        dispatcher.dispatch_later(Cycle(2), AutoStartEvent::Tick);
                    }
                }
            }
        }

        fn power_up(&self, dispatcher: &mut Dispatcher<Self::Event>){
            // Schedule boot event 1 cycle after power up
            dispatcher.dispatch_later(Cycle(1), AutoStartEvent::Boot);
        }
    }

    // Default constructor should call power_up and schedule initial events
    let mut sim: Simulator<AutoStart> = Simulator::new(FREQ);

    sim.play();

    assert_eq!(sim.simulatable.boots, 1);
    assert_eq!(sim.simulatable.ticks, 3);
    // Timing: 1(boot) + 5(tick) + 2(tick) + 2(tick) = 10
    assert_eq!(sim.now(), Cycle(10));
}

#[test]
fn test_tuple_composition() {
    // Two counters with shared event type
    #[derive(Debug, Clone, Copy, Serialize, PartialEq)]
    enum SharedEvent {
        CountA,
        CountB,
    }

    impl std::fmt::Display for SharedEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SharedEvent::CountA => write!(f, "CountA"),
                SharedEvent::CountB => write!(f, "CountB"),
            }
        }
    }

    impl Event for SharedEvent {}

    #[derive(Default, Debug, Serialize)]
    struct CounterA {
        count: usize,
    }

    #[derive(Default, Debug, Serialize)]
    struct CounterB {
        count: usize,
    }

    impl Simulatable for CounterA {
        type Event = SharedEvent;

        fn handle(
            &mut self,
            dispatcher: &mut Dispatcher<Self::Event>,
            trigger: Trigger<Self::Event>,
        ){
            match trigger.event {
                SharedEvent::CountA => {
                    self.count += 1;
                    if self.count < 2 {
                        dispatcher.dispatch_later(Cycle(3), SharedEvent::CountA);
                    }
                }
                _ => {}
            }
        }
    }

    impl Simulatable for CounterB {
        type Event = SharedEvent;

        fn handle(
            &mut self,
            dispatcher: &mut Dispatcher<Self::Event>,
            trigger: Trigger<Self::Event>,
        ){
            match trigger.event {
                SharedEvent::CountB => {
                    self.count += 10;
                    if self.count < 30 {
                        dispatcher.dispatch_later(Cycle(4), SharedEvent::CountB);
                    }
                }
                _ => {},
            }
        }
    }

    let mut sim: Simulator<(CounterA, CounterB)> = Simulator::new(FREQ);

    // Start the system
    sim.dispatch_later(Cycle(1), SharedEvent::CountA);
    sim.dispatch_later(Cycle(1), SharedEvent::CountB);

    sim.play();

    // Both components should have processed events
    assert_eq!(sim.simulatable.0.count, 2);
    assert_eq!(sim.simulatable.1.count, 30);
}

#[test]
fn test_tuple_power_up() {
    #[derive(Debug, Clone, Copy, Serialize, PartialEq)]
    enum StartEvent {
        InitEarly,
        InitLate,
    }

    impl std::fmt::Display for StartEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                StartEvent::InitEarly => write!(f, "InitEarly"),
                StartEvent::InitLate => write!(f, "InitLate"),
            }
        }
    }

    impl Event for StartEvent {}

    #[derive(Default, Debug, Serialize)]
    struct EarlyStarter {
        inits: usize,
    }

    #[derive(Default, Debug, Serialize)]
    struct LateStarter {
        inits: usize,
    }

    impl Simulatable for EarlyStarter {
        type Event = StartEvent;

        fn handle(
            &mut self,
            _dispatcher: &mut Dispatcher<Self::Event>,
            trigger: Trigger<Self::Event>,
        ){
            match trigger.event {
                StartEvent::InitEarly => {
                    self.inits += 1;
                }
                _ => {}
            }
        }

        fn power_up(&self, dispatcher: &mut Dispatcher<Self::Event>){
            dispatcher.dispatch_later(Cycle(2), StartEvent::InitEarly);
        }
    }

    impl Simulatable for LateStarter {
        type Event = StartEvent;

        fn handle(
            &mut self,
            _dispatcher: &mut Dispatcher<Self::Event>,
            trigger: Trigger<Self::Event>,
        ){
            match trigger.event {
                StartEvent::InitLate => {
                    self.inits += 1;
                }
                _ => {}
            }
        }

        fn power_up(&self, dispatcher: &mut Dispatcher<Self::Event>){
            dispatcher.dispatch_later(Cycle(5), StartEvent::InitLate);
        }
    }

    // Both components should schedule power-up events
    let mut sim: Simulator<(EarlyStarter, LateStarter)> = Simulator::new(FREQ);

    sim.play();

    // Both should have processed their init events
    assert_eq!(sim.simulatable.0.inits, 1);
    assert_eq!(sim.simulatable.1.inits, 1);
    assert_eq!(sim.now(), Cycle(5)); // Last event at cycle 5
}

#[test]
fn test_triple_tuple_composition() {
    #[derive(Debug, Clone, Copy, Serialize, PartialEq)]
    enum TripleEvent {
        Ping,
    }

    impl std::fmt::Display for TripleEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TripleEvent::Ping => write!(f, "Ping"),
            }
        }
    }

    impl Event for TripleEvent {}

    #[derive(Debug, Default, Serialize)]
    struct ComponentA {
        count: usize,
    }
    #[derive(Debug, Default, Serialize)]
    struct ComponentB {
        count: usize,
    }
    #[derive(Debug, Default, Serialize)]
    struct ComponentC {
        count: usize,
    }

    impl Simulatable for ComponentA {
        type Event = TripleEvent;
        fn handle(
            &mut self,
            _dispatcher: &mut Dispatcher<Self::Event>,
            _trigger: Trigger<Self::Event>,
        ){
            self.count += 1;
        }
    }

    impl Simulatable for ComponentB {
        type Event = TripleEvent;
        fn handle(
            &mut self,
            _dispatcher: &mut Dispatcher<Self::Event>,
            _trigger: Trigger<Self::Event>,
        ){
            self.count += 2;
        }
    }

    impl Simulatable for ComponentC {
        type Event = TripleEvent;
        fn handle(
            &mut self,
            _dispatcher: &mut Dispatcher<Self::Event>,
            _trigger: Trigger<Self::Event>,
        ){
            self.count += 3;
        }

        fn report<'t>(&self, tracer: &mut Tracer<Self::Event>) {
            tracer.add_simulatable(self);
            tracer.add_counter("ComponentC_count", self.count as f64);
        }
    }

    let mut sim: Simulator<(ComponentA, ComponentB, ComponentC)> = Simulator::new(FREQ);

    sim.dispatch_later(Cycle(1), TripleEvent::Ping);
    sim.dispatch_later(Cycle(10), TripleEvent::Ping);
    sim.dispatch_later(Cycle(20), TripleEvent::Ping);
    sim.dispatch_later(Cycle(100), TripleEvent::Ping);
    sim.dispatch_later(Cycle(200), TripleEvent::Ping);
    sim.play();
    assert_eq!(sim.simulatable.0.count, 5);
    assert_eq!(sim.simulatable.1.count, 10);
    assert_eq!(sim.simulatable.2.count, 15);
}
