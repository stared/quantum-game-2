<template>
  <AppLayout>
    <template #main>
      <div class="login-page-wrapper">
        <h1>User Login</h1>
        <p>Login with your social account</p>
        <div class="social-login">
          <div class="social-login__btn social-login__gh" @click="actionSignInGithub">Github</div>
          <div class="social-login__btn social-login__fb" @click="actionSignInFacebook">
            Facebook
          </div>
          <div class="social-login__btn social-login__g" @click="actionSignInGoogle">Google</div>
        </div>
        <p class="separator">or</p>
        <!-- <div v-if="moduleGetterError" class="alert-error">{{ moduleGetterError }}</div> -->
        <form class="email-login" action="#" @submit.prevent="actionSignIn(user)">
          <div class="email-login__email">
            <input
              id="email"
              v-model="user.email"
              type="email"
              class="form-control"
              name="email"
              value
              required
              autofocus
              placeholder="email"
            />
          </div>
          <div class="email-login__password">
            <input
              id="password"
              v-model="user.password"
              type="password"
              class="form-control"
              name="password"
              required
              placeholder="password"
            />
          </div>
          <div class="login-button-wrapper">
            <div class="rememberme">
              <input v-model="user.rememberMe" type="checkbox" checked name="rememberme" />
              <label for="rememberme">Remember Me</label>
            </div>
            <AppButton type="special"> Login </AppButton>
          </div>
        </form>
        <p>Forgot your password?</p>
        <p>Don't have an account? <router-link to="/register"> Sign Up </router-link></p>
      </div>
    </template>
  </AppLayout>
</template>

<script lang="ts">
import AppLayout from '@/components/AppLayout.vue'
import AppButton from '@/components/AppButton.vue'
import { storeNamespace } from '@/store'
import { defineComponent, reactive } from 'vue'

export default defineComponent({
  components: {
    AppLayout,
    AppButton,
  },
  setup() {
    const userModule = storeNamespace('user')
    const actionSignIn = userModule.useAction('SIGN_IN')
    const actionSignInGithub = userModule.useAction('SIGN_IN_GITHUB')
    const actionSignInFacebook = userModule.useAction('SIGN_IN_FACEBOOK')
    const actionSignInGoogle = userModule.useAction('SIGN_IN_GOOGLE')
    // FIXME: no such getter. Were all error moved to notifications?
    // const moduleGetterError = user.useGetter('error')

    const user = reactive({
      email: '',
      password: '',
      rememberMe: true,
    })

    return { actionSignIn, actionSignInGithub, actionSignInFacebook, actionSignInGoogle, user }
  },
})
</script>

<style lang="scss" scoped>
.login-page-wrapper {
  max-width: 320px;
  margin: 80px auto 0;
}
h1 {
  color: white;
  font-size: 1.5rem;
  font-weight: 900;
  margin-bottom: 30px;
  line-height: 150%;
  text-transform: uppercase;
}
p {
  color: rgba(255, 255, 255, 0.7);
}

// .alert-error {
//   margin-bottom: 20px;
//   color: #ff0055;
// }

.separator {
  position: relative;
}

.separator::before,
.separator::after {
  content: '';
  position: absolute;
  border-top: 2px solid #fff;
  width: 45%;
  top: 50%;
  right: 55%;
}

.separator::after {
  left: 55%;
}

.social-login__btn {
  width: 100%;
  padding: 10px 0;
  margin-bottom: 5px;
  cursor: pointer;
}

.social-login {
  &__gh {
    background: #120223;
  }
  &__fb {
    background: #39579a;
  }
  &__g {
    background: #4285f4;
  }
}

input[type='email'],
input[type='password'] {
  outline: none;
  display: inline-block;
  width: 100%;
  padding: 10px;
  background: #fff;
  border-top: none;
  border-right: none;
  border-left: none;
  border-bottom: none;
  box-sizing: border-box;
  color: #000;
  font-size: 1rem;
  font-weight: normal;
  -webkit-box-sizing: border-box;
  -moz-box-sizing: border-box;
  -o-box-sizing: border-box;
  -ms-box-sizing: border-box;
  margin-bottom: 5px;
}

input[type='email']:focus,
input[type='password']:focus {
  color: #000;
  font-size: 1rem;
  font-weight: normal;
  border-top: none;
  border-right: none;
  border-left: none;
  border-bottom: none;
  box-sizing: border-box;
}

::placeholder {
  color: rgba(0, 0, 0, 0.4);
  text-align: center;
}
.login-button-wrapper {
  display: flex;
  justify-content: space-between;
  width: 100%;
  align-items: center;
  margin-top: 25px;
}

.rememberme label {
  font-size: 1rem;
  color: rgba(255, 255, 255, 0.7);
}

@media screen and (max-width: 736px) {
  input[type='email']:focus,
  input[type='password']:focus {
    width: 100%;
  }
}
</style>
