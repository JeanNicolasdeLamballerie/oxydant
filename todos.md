# Oxydant's todos

## Features and bugs

- The first question is, do we want a transpiler, a compiler, or both ?
The rules for each would be different.

- The second question is, how do we multithread it ? I'd assume analyzing each
file in one thread.
Then waiting for answers from other threads (either by resuming operation in a
new thread
or by referring to ArcMutexes of shared memory) if there are cross-references
would be the best.
