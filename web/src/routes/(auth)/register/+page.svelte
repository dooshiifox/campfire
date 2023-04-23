<script lang="ts">
	import { Account } from '$/sdk';
	import { user } from '$/state';

	let username = '';
	let email = '';
	let password = '';
	let passwordConfirm = '';

	async function register() {
		if (password !== passwordConfirm) {
			alert('Passwords do not match.');
			return;
		}

		const register = await Account.register({
			username,
			email,
			password
		})
			.emptyOnError()
			.send();

		if (register) {
			$user = {
				accessToken: register.access_token,
				user: register.user
			};
			window.location.href = '/';
		}
	}
</script>

<div class="grid h-screen w-screen place-items-center bg-zinc-800">
	<div class="w-full max-w-lg rounded-lg bg-zinc-900 px-6 py-4">
		<h1 class="text-sm font-bold uppercase text-zinc-400">Register</h1>

		<form on:submit={register}>
			<label for="email" class="mt-6 block text-zinc-400">Username</label>
			<input
				id="username"
				type="text"
				bind:value={username}
				class="mt-1 block w-full rounded border border-zinc-700 bg-zinc-800 px-4 py-1 text-lg text-zinc-300 focus:border-zinc-600 focus:outline-none"
			/>

			<label for="email" class="mt-4 block text-zinc-400">Email</label>
			<input
				id="email"
				type="email"
				bind:value={email}
				class="mt-1 block w-full rounded border border-zinc-700 bg-zinc-800 px-4 py-1 text-lg text-zinc-300 focus:border-zinc-600 focus:outline-none"
			/>

			<label for="password" class="mt-4 block text-zinc-400">Password</label>
			<input
				id="password"
				type="password"
				bind:value={password}
				class="mt-1 block w-full rounded border border-zinc-700 bg-zinc-800 px-4 py-1 text-lg text-zinc-300 focus:border-zinc-600 focus:outline-none"
			/>
			<label for="password-confirm" class="mt-2 block text-zinc-400">Confirm Password</label>
			<input
				id="password-confirm"
				type="password"
				bind:value={passwordConfirm}
				class="mt-1 block w-full rounded border border-zinc-700 bg-zinc-800 px-4 py-1 text-lg text-zinc-300 focus:border-zinc-600 focus:outline-none"
			/>

			<div class="mt-4 flex flex-row items-center">
				<!-- <input id="show-password" type="checkbox" bind:checked={showPassword} />
				<label for="show-password" class="pl-4 text-zinc-400">Show Password</label> -->

				<span class="grow" />

				<a
					href="/register"
					class="cursor-pointer px-4 text-right font-bold text-zinc-300 hover:underline"
				>
					Login
				</a>
			</div>

			<div class="mt-6 flex justify-center gap-4">
				<input
					type="submit"
					class="block w-1/2 cursor-pointer rounded bg-amber-700 px-4 py-2 font-bold uppercase text-amber-50 hover:bg-amber-600 disabled:opacity-20 disabled:hover:bg-amber-700"
					value="Register"
					disabled={!username || !email || !password || !passwordConfirm}
				/>
			</div>
		</form>
	</div>
</div>
