// FIXME: Figure a way to have uid and coord access to cells
// FIXME: Figure out blank cells in constructor
import * as qt from 'quantum-tensors';
import { CellInterface, GridInterface, ParticleInterface } from './interfaces';
import Coord from './Coord';
import Element from './Element';
import Cell from './Cell';
import Cluster from './Cluster';
import Particle from './Particle';
import { flatDeep } from './Helpers';

/**
 * Grid class includes the grid instance that holds the cells
 */
export default class Grid extends Cluster {
  public cols: number;
  public rows: number;
  public paths: Particle[];

  constructor(rows: number, cols: number, cells?: Cell[]) {
    super(cells);
    this.rows = rows;
    this.cols = cols;

    // Populate with blank tiles
    for (let y = 0; y < rows; y += 1) {
      for (let x = 0; x < cols; x += 1) {
        const coord = Coord.importCoord({ y, x });
        const element = Element.fromName('Void');
        const cell = new Cell(coord, element);
        this.cells.push(cell);
      }
    }

    // If cells are given compute the laser path
    this.paths = this.computePaths();
  }

  /**
   * Set a cell at a specific coordinate
   * @param cell Cell to set at a grid coordinate
   * @returns boolean if operation is successfull
   */
  public set(cell: Cell): boolean {
    if (this.includes(cell.coord)) {
      const currentCell = this.get(cell.coord);
      currentCell.element = cell.element;
      currentCell.frozen = cell.frozen;
      currentCell.active = cell.active;
      currentCell.energized = cell.energized;
      currentCell.rotation = cell.rotation;
      currentCell.tool = cell.tool;
      return true;
    }
    throw new Error(`Coordinate out of bounds. Cell: [${cell.coord.x}, ${cell.coord.y}]`);
  }

  /**
   * Retrieve the cell at a specified coordinate
   * @param coord Coordinate to get
   * @returns Cell
   */
  public get(coord: Coord): Cell {
    return this.cells.filter((cell) => {
      return coord.equal(cell.coord);
    })[0];
  }

  /**
   * Get center cell of the grid
   * @returns center cell coordinates
   */
  get center(): Coord {
    return Coord.importCoord({
      y: Math.floor(this.cols / 2),
      x: Math.floor(this.rows / 2)
    });
  }

  /**
   * Remove unfrozen cells once they are moved to the toolbox
   */
  resetUnfrozen(): void {
    this.unfrozen.cells.forEach((cell) => {
      cell.reset();
    });
  }

  /**
   * Retrieve the list of quantum operators from the elements
   * @returns list of operators
   */
  get operatorList(): [number, number, qt.Operator][] {
    return this.unvoid.cells.map((cell) => {
      return [cell.coord.x, cell.coord.y, cell.element.transition(cell.rotation)];
    });
  }

  /**
   * Is a coordinate inside the grid
   * @param coord Coordiante to test
   * @returns boolean if included
   */
  public includes(coord: Coord): boolean {
    return coord.y >= 0 && coord.y < this.rows && (coord.x >= 0 && coord.x < this.cols);
  }

  /**
   * Move a cell to another coord
   * @param sourceCell source cell
   * @param targetCell target cell
   * @returns boolean move was successfull
   */
  public move(sourceCell: Cell, targetCell: Cell): boolean {
    const source = sourceCell;
    const target = targetCell;
    if (source.isFromGrid && source.tool && target.isFromGrid && target.isVoid) {
      const tempCoord = source.coord;
      source.coord = target.coord;
      target.coord = tempCoord;
      target.tool = false;
      source.tool = true;
      this.set(source);
      this.set(target);
      return true;
    }

    // SWAP GRID TOOL TO GRID TOOL
    if (source.isFromGrid && source.tool && target.isFromGrid && target.tool) {
      const tempCoord = source.coord;
      source.coord = target.coord;
      target.coord = tempCoord;
      target.tool = true;
      source.tool = true;
      this.set(source);
      this.set(target);
      return true;
    }

    // MOVE TOOLBOX TOOL TO GRID VOID
    if (source.isFromToolbox && source.tool && target.isFromGrid && target.isVoid) {
      target.element = source.element;
      target.tool = true;
      this.set(target);
      return true;
    }

    return false;
  }

  /**
   * Move all elements to a common direction
   * @param direction direction string
   */
  public moveAll(direction: number): void {
    console.debug(`Moving all in direction: ${direction}`);
    this.cells.map((cell) => {
      return cell.coord.fromAngle(direction);
    });
  }

  /**
   * Fire all the lasers
   * @returns the particles fired
   */
  public fireLasers(): Particle[] {
    return this.lasers.active.cells.map((laser) => {
      if (laser.active) {
        return new Particle(laser.coord, laser.rotation, 1, 0);
      }
      throw Error('Laser is inactive...');
    });
  }

  /**
   * Compute the classical intensity using laser paths of a coordinate
   * FIXME: Move to level or to grid
   * @param coord Coordinate
   */
  coordIntensitySum(coord: Coord): number {
    return this.paths
      .filter((particleInterface) => {
        return coord.equal(particleInterface.coord);
      })
      .reduce((a, b) => a + b.probability, 0);
  }

  /**
   * Compute the laser path of a particle
   * @param particle Particle which needs its laser path computed
   * @param maxFrames Max number of frames to compute
   * @returns list of "path particles"
   */
  laserPath(initParticle: Particle, maxFrames = 40): Particle[][] {
    // Make a depp clone of the particle
    let alive: Particle[] = [initParticle];
    const dead: Particle[] = [];

    // Simulate path with a specific number of frames
    for (let i = 0; i < maxFrames; i += 1) {
      // Propagate each living particle
      // eslint-disable-next-line
      alive.forEach((particle: Particle) => {
        particle.next();

        // Zero the intensity of escaping particles
        if (!this.includes(particle.coord)) {
          // eslint-disable-next-line no-param-reassign
          particle.intensity = 0;
        }

        // Absorption
        this.absorbers.cells.forEach((absorber: Cell) => {
          if (particle.on(absorber)) {
            // eslint-disable-next-line no-param-reassign
            particle.intensity -= particle.intensity * absorber.element.absorption;
          }
        });

        // Reflection
        this.mirrors.cells.forEach((mirror: Cell) => {
          if (particle.on(mirror)) {
            // eslint-disable-next-line no-param-reassign
            particle.direction = (2 * mirror.rotation - particle.direction + 360) % 360;
          }
        });
        this.polarbeamsplitters.cells.forEach((polar: Cell) => {
          if (particle.on(polar)) {
            if (polar.rotation === 0) {
              const direction = (2 * (polar.rotation - 45) - particle.direction + 360) % 360;
              alive.push(new Particle(particle.coord, direction, particle.intensity));
            }
            if (polar.rotation === 180) {
              const direction = (2 * (polar.rotation + 45) - particle.direction + 360) % 360;
              alive.push(new Particle(particle.coord, direction, particle.intensity));
            }
          }
        });
        this.beamsplitters.cells.forEach((beamsplitter: Cell) => {
          if (particle.on(beamsplitter)) {
            // Dim the current particle intensity
            // eslint-disable-next-line no-param-reassign
            particle.intensity /= 2;
            // Reflecting particle (create new reflected faded particle)
            const direction = (2 * beamsplitter.rotation - particle.direction + 360) % 360;
            alive.push(new Particle(particle.coord, direction, particle.intensity));
          }
        });
      });

      // Filter the living from the dead
      alive.forEach((particle) => {
        if (!particle.alive) {
          dead.push(particle);
        }
      });
      alive = alive.filter((particle) => {
        return particle.alive;
      });
    }

    // Flatten and dedupe list of particles
    const pathParticles: Particle[][] = [];
    alive = dead.concat(alive);
    alive.forEach((particle) => {
      pathParticles.push(particle.pathParticle);
    });
    return pathParticles;
    // return [...new Set(flatDeep(pathParticles))]
  }

  /**
   * Gives the classical laser path of a specific particle
   * FIXME: Could be refactored
   * @returns a list of coordinates
   * */
  computePaths(): Particle[] {
    const laserParticles = this.fireLasers();
    const particles = laserParticles.map((laserParticle: Particle) => {
      return [...new Set(this.laserPath(laserParticle, 40).flat())];
    });
    return particles.flat();
  }

  /**
   * Set the cells as energized if on this laser path.
   * @param paths laser path to energize
   */
  energizeCells(paths: ParticleInterface[]): void {
    const pathCoords: Coord[] = paths.map((pathParticle) => Coord.importCoord(pathParticle.coord));
    this.cells.forEach((cell: Cell) => {
      if (cell.coord.isIncludedIn(pathCoords) && cell.element.name !== 'Void') {
        // eslint-disable-next-line no-param-reassign
        cell.energized = true;
      } else {
        // eslint-disable-next-line no-param-reassign
        cell.energized = false;
      }
    });
  }

  /**
   * Set the adjacent cells as active if they are near an energized detector
   */
  activateCells(): void {
    this.unvoid.cells.forEach((cell) => {
      if (cell.element.name !== 'laser') {
        // eslint-disable-next-line no-param-reassign
        cell.active = false;
      }
      const energizedAdjacent = this.adjacentCells(cell.coord).filter((adjacent) => {
        return adjacent.energized && adjacent.element.name === 'detector';
      });
      if (energizedAdjacent.length > 0) {
        console.debug(`Cell ${cell.toString()} has 1+ active detectors as adjacent cell.`);
        // eslint-disable-next-line no-param-reassign
        cell.active = true;
      }
    });
  }

  /**
   * Return adjacent cells to a coordinate
   * @param coord Coordinate
   * @returns a list of adjacent cells
   */
  adjacentCells(coord: Coord): Cell[] {
    const adjacents: Cell[] = [];
    coord.adjacent.forEach((adjacent) => {
      if (this.includes(adjacent)) {
        adjacents.push(this.get(adjacent));
      }
    });
    return adjacents;
  }

  /**
   * Output an ASCII grid
   * @returns an ascii grid
   */
  public get ascii(): string {
    let result = '';
    for (let y = 0; y < this.rows; y += 1) {
      for (let x = 0; x < this.cols; x += 1) {
        const coord = Coord.importCoord({ y, x });
        result += this.get(coord).ascii;
      }
      result += '\n';
    }
    return result;
  }

  /**
   * Sets the grid with the appropriate cells
   * @param jsonCells A list of cell interface
   */
  public importGrid(jsonCells: CellInterface[]): void {
    jsonCells.forEach((jsonCell) => {
      const cell = Cell.importCell(jsonCell);
      this.set(cell);
    });
  }

  /**
   * Exports the grid to an interface of primitives
   * @returns a grid interface
   */
  public exportGrid(): GridInterface {
    const cells: CellInterface[] = [];
    this.cells
      .filter((cell) => !cell.isVoid)
      .forEach((cell) => {
        cells.push(cell.exportCell());
      });
    return {
      cols: this.cols,
      rows: this.rows,
      cells
    };
  }
}
