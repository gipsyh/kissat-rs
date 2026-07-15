cmake_minimum_required(VERSION 3.10)
project(for-rIC3 LANGUAGES C)
set(CMAKE_C_FLAGS_RELEASE "-O3")

set(S kissat/src/)
add_library(satif-kissat
  ${S}allocate.c ${S}analyze.c ${S}ands.c ${S}arena.c ${S}assign.c
  ${S}averages.c ${S}backbone.c ${S}backtrack.c ${S}bump.c ${S}check.c
  ${S}classify.c ${S}clause.c ${S}collect.c ${S}compact.c ${S}config.c
  ${S}congruence.c ${S}decide.c ${S}deduce.c ${S}definition.c ${S}dense.c
  ${S}dump.c ${S}eliminate.c ${S}equivalences.c dummy.c ${S}extend.c
  ${S}factor.c ${S}fastel.c ${S}flags.c ${S}format.c ${S}forward.c ${S}gates.c
  ${S}heap.c ${S}ifthenelse.c ${S}import.c ${S}internal.c ${S}kimits.c
  ${S}kitten.c ${S}krite.c ${S}learn.c ${S}logging.c ${S}lucky.c ${S}minimize.c
  ${S}mode.c ${S}options.c ${S}phases.c ${S}preprocess.c ${S}print.c
  ${S}probe.c ${S}promote.c ${S}proof.c ${S}propbeyond.c ${S}propdense.c
  ${S}propinitially.c ${S}proprobe.c ${S}propsearch.c ${S}queue.c ${S}reduce.c
  ${S}reluctant.c ${S}reorder.c ${S}rephase.c ${S}report.c ${S}resize.c
  ${S}resolve.c ${S}restart.c ${S}search.c ${S}shrink.c ${S}smooth.c ${S}sort.c
  ${S}stack.c ${S}statistics.c ${S}strengthen.c ${S}substitute.c ${S}sweep.c
  ${S}terminate.c ${S}tiers.c ${S}trail.c ${S}transitive.c ${S}utilities.c
  ${S}vector.c ${S}vivify.c ${S}walk.c ${S}warmup.c ${S}watch.c ${S}weaken.c
)

target_compile_definitions(satif-kissat PRIVATE QUIET NOPTIONS NPROOFS NDEBUG)
install(TARGETS satif-kissat ARCHIVE DESTINATION lib LIBRARY DESTINATION lib)
