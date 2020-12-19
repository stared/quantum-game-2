<template>
  <div class="interactive">
    <EncyclopediaBoard :grid="boardMZ" :step="5" class="grid" :defaultStep="2" />
  </div>
  <p>
    For more interactive, educational materials, see and
    <a href="https://github.com/Quantum-Game/bra-ket-vue" target="_blank"
      >⟨𝜑|𝜓⟩.vue visualizer for quantum states and matrices</a
    >, which can be used for quantum computing, quantum chemistry or any other discrete quantum
    system.
  </p>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import EncyclopediaBoard from '@/components/EncyclopediaPage/EncyclopediaBoard.vue'
import { getEntry } from '@/components/EncyclopediaPage/loadData'

export default defineComponent({
  components: {
    EncyclopediaBoard,
  },
  setup() {
    const boardMZ = getEntry('mach-zehnder-interferometer').grids[0]
    return { boardMZ }
  },
})
</script>

<style lang="scss" scoped>
a {
  color: #eab5ff;
  text-decoration: none;
}
a:hover {
  color: white;
}
.interactive {
  background: #2e006a;
  padding: 30px;
}
</style>
