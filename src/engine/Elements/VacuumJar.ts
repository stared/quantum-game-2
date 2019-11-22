import * as qt from 'quantum-tensors';
import { Elem, Group } from '@/engine/interfaces';
import Element from '@/engine/Element';

/**
 * VACUUM JAR CLASS
 */
export default class VacuumJar extends Element {
  name: Elem = Elem.VacuumJar;
  group: Group = Group.Phase;
  description: string =
    'Even air retards light a bit. We set the thickness of vacuum so it advances the phase by λ/4. Useful for changing interference.';
  ascii: string[] = ['↝'];
  angles: number[] = [0];

  constructor() {
    super(Elem.VacuumJar, Group.Phase);
  }

  transition() {
    return qt.vacuumJar();
  }
}