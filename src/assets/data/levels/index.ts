import { ILevel } from '@/engine/interfaces'
import introLevels from './introLevels.json'
import reflectionLevels from './reflectionLevels.json'
import phaseLevels from './phaseLevels.json'
import interferenceLevels from './interferenceLevels.json'
import polarizationLevels from './polarizationLevels.json'
import finalLevels from './finalLevels.json'
import classicLevels from './classicLevels.json'
import sandbox from './sandboxLevel.json'
import benchmarkLevels from './benchmark.json'

export {
  sandbox,
  introLevels,
  reflectionLevels,
  phaseLevels,
  interferenceLevels,
  polarizationLevels,
  finalLevels,
  classicLevels,
  benchmarkLevels,
}

const introGroup: ILevel[] = introLevels
const reflectionGroup: ILevel[] = reflectionLevels
const phaseGroup: ILevel[] = phaseLevels
const interferenceGroup: ILevel[] = interferenceLevels
const polarizationGroup: ILevel[] = polarizationLevels
const finalGroup: ILevel[] = finalLevels
const classicGroup: ILevel[] = classicLevels
const benchmarkGroup: ILevel[] = benchmarkLevels

const levels: ILevel[] = [
  sandbox,
  ...classicGroup,
  ...introGroup,
  ...reflectionGroup,
  ...phaseGroup,
  ...interferenceGroup,
  ...polarizationGroup,
  ...finalGroup,
  ...benchmarkGroup,
]

export default levels
