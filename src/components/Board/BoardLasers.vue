<template>
  <!-- LASER PATH -->
  <g class="lasers">
    <g v-for="(laser, index) in paths" :key="'laser' + index" :v-if="paths.length > 0">
      <path
        :d="laser"
        stroke-dasharray="8 8"
        fill="transparent"
        stroke="red"
        stroke-width="3"
        class="laserPath"
      />
    </g>
  </g>
</template>

<script lang="ts">
import { Vue, Prop, Component } from 'vue-property-decorator';
import { State } from 'vuex-class';
import { ParticleInterface } from '@/engine/interfaces';
import Grid from '@/engine/Grid';
import Level from '@/engine/Level';

@Component
export default class Board extends Vue {
  @Prop({ default: '' }) readonly paths!: string[];
  tileSize: number = 64;
}
</script>

<style lang="scss" scoped>
.lasers {
  width: 100%;
  height: 100%;
  .laserPath {
    stroke-dasharray: 8;
    animation-name: dash;
    animation-duration: 4s;
    animation-timing-function: linear;
    animation-iteration-count: infinite;
    animation-direction: reverse;
  }
  @keyframes dash {
    to {
      stroke-dashoffset: 64;
    }
  }
}
</style>