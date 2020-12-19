<template>
  <div class="home-page-wrapper" layout="column center">
    <div class="row-full">
      <img src="@/assets/graphics/QG_logo.svg" alt="QuantumGame" />
    </div>
    <div layout="row wrap middle u5">
      <AppButton class="button" type="big inactive">PLAY GAME (coming 2021)</AppButton>
      <a href="https://lab.quantumflytrap.com">
        <AppButton class="button" type="big">VIRTUAL LAB (alpha version)</AppButton>
      </a>
    </div>
    <div class="hello" layout="column u10">
      <h1>{{ i18n.t('homepage.main') }}</h1>
      <p class="main-description">{{ i18n.t('homepage.desc') }}</p>
      <p>
        More on the project at the
        <a href="https://quantumflytrap.com">Quantum Flytrap</a> homepage.
      </p>
      <HomepageInteractive />
    </div>

    <div class="hello" layout="column u10">
      <HomepageTestimonials class="testimonials" />
      <HomepageSocial />
      <HomepageAuthors />
      <HomepageHistory />
    </div>
    <svg viewBox="0 0 64 64" width="200" height="200">
      <AppCell :piece="piece" class="rock" />
    </svg>
  </div>
</template>

<script lang="ts">
import AppButton from '@/components/AppButton.vue'
import HomepageAuthors from '@/components/HomePage/HomepageAuthors.vue'
import HomepageSocial from '@/components/HomePage/HomepageSocial.vue'
import HomepageTestimonials from '@/components/HomePage/HomepageTestimonials.vue'
import HomepageInteractive from '@/components/HomePage/HomepageInteractive.vue'
import HomepageHistory from '@/components/HomePage/HomepageHistory.vue'
import AppCell from '@/components/Board/AppCell.vue'
import { Elem, defaultPiece } from '@/engine/model'
import { defineComponent } from 'vue'
import { useI18n } from 'vue-i18n'

export default defineComponent({
  components: {
    AppButton,
    HomepageAuthors,
    HomepageSocial,
    HomepageHistory,
    HomepageTestimonials,
    HomepageInteractive,
    AppCell,
  },
  setup() {
    return {
      i18n: useI18n(),
      version: process.env.VUE_APP_VERSION || 'unknown',
      commitHash: process.env.VUE_APP_GIT_HASH,
      commitDate: new Date(+(process.env.VUE_APP_GIT_DATE || 0) * 1000).toUTCString(),
      piece: defaultPiece(Elem.Rock),
    }
  },
})
</script>

<style lang="scss" scoped>
.home-page-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  background: linear-gradient(#5c00d3, #f05, #5c00d3, #210235);
  width: 100%;
  & a {
    & img {
      max-width: 250px;
    }
  }
}
.gameVersion {
  font-size: 0.8rem;
  margin: 20px 10px;
  @include media('<large') {
    font-size: 1rem;
    width: 80%;
  }
}
h1 {
  color: white;
  font-size: 1.4rem;
  font-weight: 900;
  line-height: 150%;
  margin-top: 50px;
}
p {
  color: white;
  line-height: 150%;
}
.main-description {
  font-size: 1.4rem;
}
.hello {
  width: 35%;
  display: flex;
  position: center;
  flex-direction: column;
  align-items: center;
  @media screen and (max-width: 1259px) {
    width: 60%;
  }
  @media screen and (max-width: 560px) {
    width: 80%;
  }
}

.row-full {
  width: 100vw;
  min-width: 1300px;
  position: center;
  height: 100px;
  margin-bottom: 8rem;
  align-items: center;
  @include media('<large') {
    margin-bottom: 6rem;
  }
}

.testimonials {
  max-width: 1300px;
  width: 100vw;
}

a {
  color: #eab5ff;
  text-decoration: none;
}
a:hover {
  color: white;
}

* {
  transition: all 0.6s;
}

.rock {
  cursor: default !important;
}

::placeholder {
  color: white;
}
</style>
