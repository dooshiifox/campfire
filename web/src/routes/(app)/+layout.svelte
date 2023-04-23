<script lang="ts">
	import { page } from '$app/stores';
	import { Helper } from '$/sdk';
	import { errors, guilds } from '$/state';
</script>

<div class="flex h-screen w-screen flex-row bg-zinc-800">
	<div class="flex w-16 shrink-0 flex-col bg-zinc-900 pt-2">
		{#each $guilds as guild (guild.id)}
			{@const onThisGuild =
				$page.route.id === '/(app)/channel/[guild]/[channel]' && $page.params.guild === guild.id}
			<a class="group relative w-16 p-2" href="/channel/{guild.id}/{guild.channels[0]?.id}">
				<div
					class="aspect-square w-full {onThisGuild
						? 'rounded-[33%]'
						: 'rounded-[50%]'} overflow-hidden transition-[border-radius] group-hover:rounded-[33%]"
				>
					<img src={Helper.guildIcon(guild)} class="h-full w-full bg-white" alt={guild.name} />
				</div>
				<div
					class="absolute inset-y-0 left-0 {onThisGuild
						? 'w-1'
						: 'w-0'} flex flex-row items-center transition-[width] group-hover:w-1"
				>
					<div
						class="grow rounded-r-full {onThisGuild
							? 'h-10 bg-zinc-200'
							: 'h-4 bg-white'} transition-[height,background-color] group-hover:h-10"
					/>
				</div>
			</a>
		{/each}
	</div>

	<slot />
</div>
