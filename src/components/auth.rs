use leptos::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn LoginPage(
    cx: Scope,
    action: Action<crate::api::auth::Login, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
      <div class="mx-auto max-w-lg">
        <h1 class="text-center text-2xl font-bold text-indigo-600 sm:text-3xl">
          "开始学习每一天"
        </h1>

        <ActionForm
          action=action
          class="mt-6 mb-0 space-y-4 rounded-lg p-4 shadow-lg sm:p-6 lg:p-8"
        >
          <p class="text-center text-lg font-medium">"Sign in to your account"</p>

          <div>
            <label for="username" class="sr-only">"用户名"</label>

            <div class="relative">
              <input
                type="text" name="username"
                class="w-full rounded-lg border-gray-200 p-4 pr-12 text-sm shadow-sm"
                placeholder="此处输入用户名"
              />
            </div>
          </div>

          <div>
            <label for="password" class="sr-only">"密码"</label>

            <div class="relative">
              <input
                type="password" name="password"
                class="w-full rounded-lg border-gray-200 p-4 pr-12 text-sm shadow-sm"
                placeholder="此处输入密码"
              />
            </div>
          </div>

          <button
            type="submit"
            class="block w-full rounded-lg bg-indigo-600 px-5 py-3 text-sm font-medium text-white"
          >
            "登录"
          </button>

        </ActionForm>
      </div>
    </div>
        }
}
