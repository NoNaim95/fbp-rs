use crate::*;
use channels::receivers::*;
use channels::senders::*;

use crate::components::Component;

trait Scheduler {
    fn connect_processor_to_processor<T, S, R, LeftComp, RightComp, OutputComp>(
        lhs: LeftComp,
        rhs: RightComp,
        pipe: (S, R),
    ) -> OutputComp
    where
        LeftComp: ProcessorComponent<O = T>,
        RightComp: ProcessorComponent<I = T>,
        OutputComp: ProcessorComponent<I = LeftComp::I, O = RightComp::O>,
        S: Sender<T>,
        R: Receiver<T>;

    fn connect_ioc_to_component<T, S, R, LeftComp, RightComp, OutputComp>(
        lhs: LeftComp,
        rhs: RightComp,
        pipe: (S, R),
    ) -> OutputComp
    where
        LeftComp: IocGeneratorComponent<O = T>,
        RightComp: Component<I = T>,
        OutputComp: ProcessorComponent<I = (), O = RightComp::O>,
        S: Sender<T>,
        R: Receiver<T>;
}
