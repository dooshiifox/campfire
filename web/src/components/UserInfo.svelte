<!-- @component
    The user info is the user's avatar, username, and discriminator displayed
    at the bottom of the screen where the channels are located.
-->
<script lang="ts">
	import { Helper } from '$/sdk';
	import { user } from '$/state';
	import { Popover, PopoverButton, PopoverPanel, Transition } from '@rgossiaux/svelte-headlessui';
	import { createPopperActions } from 'svelte-popperjs';

	const [popperRef, popperContent] = createPopperActions();
</script>

<div class="shrink-0 border-t border-zinc-900">
	<Popover>
		<PopoverButton
			use={[popperRef]}
			as="div"
			class="mx-2 my-1 flex cursor-pointer select-none flex-row items-center gap-4 rounded p-2 hover:bg-zinc-700"
		>
			<div class="aspect-square w-8">
				<img
					src={Helper.profileImage($user?.user)}
					class="h-full w-full overflow-clip rounded-full bg-white"
					alt="Your Avatar"
				/>
			</div>
			<div class="flex flex-col">
				<div class="flex flex-row items-baseline gap-2">
					<span class="text-sm font-bold leading-4 text-zinc-100">{$user?.user.username}</span>
				</div>
				<div class="flex flex-row items-baseline gap-2">
					<span class="text-xs leading-3 text-zinc-400">
						#{$user?.user.discrim.toString().padStart(4, '0')}
					</span>
				</div>
			</div>
		</PopoverButton>

		<Transition
			enter="transition duration-100 ease-in"
			enterFrom="opacity-0 translate-y-2"
			enterTo="opacity-100 translate-y-0"
			leave="transition duration-75 ease-out"
			leaveFrom="opacity-100 translate-y-0"
			leaveTo="opacity-0 translate-y-2"
			class="relative"
		>
			<PopoverPanel>
				<div
					class="flex w-56 rounded bg-zinc-900 p-2"
					use:popperContent={{
						placement: 'bottom-start',
						modifiers: [{ name: 'offset', options: { offset: [0, 10] } }]
					}}
				>
					<button
						class="block w-full rounded px-4 py-1 text-start font-bold text-zinc-300 hover:bg-stone-800 hover:text-rose-200"
						on:click={() => {
							$user = undefined;
							window.location.href = '/login';
						}}
					>
						Logout
					</button>
				</div>
			</PopoverPanel>
		</Transition>
	</Popover>
</div>
