/* tslint:disable */
/* eslint-disable */
/**
* @param {number} idx
*/
export function rs_watch_particle(idx: number): void;
/**
* @param {number} particle_num
* @param {number} median_velocity
* @param {number} mass
* @param {number} color_1
* @param {number} color_2
* @param {number} color_3
*/
export function rs_add_particles(particle_num: number, median_velocity: number, mass: number, color_1: number, color_2: number, color_3: number): void;
/**
* @param {number} subiter
* @param {number} timestep
*/
export function rs_next_step(subiter: number, timestep: number): void;
/**
* @param {number} left
* @param {number} right
* @param {number} top
* @param {number} bottom
*/
export function set_borders(left: number, right: number, top: number, bottom: number): void;
