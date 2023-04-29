<script lang="ts">
	import { fade } from 'svelte/transition';

	let textWidth: number;
	let textHeight: number;

	function randInt(min: number, max: number) {
		return Math.floor(Math.random() * (max - min + 1)) + min;
	}
	function biasedRandInt(min: number, max: number) {
		const rng = 4 * (Math.random() - 0.5) ** 3 + 0.5;
		return Math.floor(rng * (max - min + 1)) + min;
	}

	function generatePath(minYPerc: number, maxYPerc: number) {
		let paths = `M0 ${textHeight}`;
		// Shifts the X position of the flame by a random amount
		const X_VARIATION = 3;
		for (let i = 0; i < textWidth + 5; i += 5) {
			const variationX = randInt(0, X_VARIATION - 1) - (X_VARIATION - 1) / 2;
			const x = i + variationX;
			const y = biasedRandInt(minYPerc * textHeight, maxYPerc * textHeight);
			paths += `L${x} ${y}`;
		}

		return paths + `V${textHeight}H0z`;
	}

	function generateAnimation(minYPerc: number, maxYPerc: number) {
		let values = '';
		for (let i = 0; i < 30; i++) {
			values += generatePath(minYPerc, maxYPerc) + ';';
		}
		return values;
	}
</script>

<div class="relative mb-20 grid grow place-items-center">
	<div class="flex flex-col items-center">
		<!-- <h1 class="text-4xl font-bold">+page</h1> -->
		<div class="absolute inset-0 -z-50">
			<h4
				class="font-brand /text-transparent w-fit select-none text-3xl font-bold"
				bind:clientWidth={textWidth}
				bind:clientHeight={textHeight}
			>
				Campfire
			</h4>
		</div>
		<!--
        This is the animated Campfire text.
        Whats here:
            - The text which is used as a clip path
            - A mask which animates to fade the top of the text a little
            - 3 paths which are animated to look like a flame, each with
                their own slight linear gradients.
            - A background which is a linear gradient of the 3 flame colors
                - The orange and red offset points animate
                - The x, y, width, and height of this box animate
                - The opacity and blur of this box animate
        All-in-all these animations create a pretty cool effect.
        Anyways, its 11pm and Ive been making this for several hours at this point.
        Bye-bye
        -->
		{#if textHeight !== undefined && textWidth !== undefined}
			<div class="relative" in:fade={{ duration: 1000 }}>
				<svg width={textWidth} height={textHeight}>
					<defs>
						<clipPath id="text">
							<text
								x="0"
								y="0"
								dominant-baseline="hanging"
								class="font-brand text-3xl font-bold text-transparent"
							>
								Campfire
							</text>
						</clipPath>
						<linearGradient id="fadeMaskGradient" gradientTransform="rotate(90)">
							<stop offset="0%" stop-color="black">
								<animate
									attributeName="offset"
									dur="3s"
									repeatCount="indefinite"
									values="0; 0; -0.1; -0.2; -0.15; -0.05; -0.1; 0"
								/>
							</stop>
							<stop offset="20%" stop-color="white">
								<animate
									attributeName="offset"
									dur="4s"
									repeatCount="indefinite"
									values="0.4; 0.2; 0.35; 0.45; 0.25; 0.15; 0.1; 0.4"
								/>
							</stop>
							<stop offset="100%" stop-color="white" />
						</linearGradient>
						<mask id="fadeMask">
							<rect
								x="0"
								y="0"
								width={textWidth}
								height={textHeight}
								fill="url(#fadeMaskGradient)"
							/>
						</mask>
						<linearGradient id="red" gradientTransform="rotate(90)">
							<stop offset="0%" stop-color="#e99c9c" />
							<stop offset="100%" stop-color="#d8a899" />
						</linearGradient>
						<linearGradient id="orange" gradientTransform="rotate(90)">
							<stop offset="0%" stop-color="#eeb39c" />
							<stop offset="100%" stop-color="#d8c89d" />
						</linearGradient>
						<linearGradient id="yellow" gradientTransform="rotate(90)">
							<stop offset="0%" stop-color="#eeda99" />
							<stop offset="100%" stop-color="#d3d29d" />
						</linearGradient>
					</defs>
					<g clip-path="url(#text)" mask="url(#fadeMask)">
						<path fill="url(#red)">
							<animate
								attributeName="d"
								dur="3s"
								repeatCount="indefinite"
								values={generateAnimation(0.2, -0.1)}
							/>
						</path>
						<path fill="url(#orange)">
							<animate
								attributeName="d"
								dur="3s"
								repeatCount="indefinite"
								values={generateAnimation(0.6, 0.3)}
							/>
						</path>
						<path fill="url(#yellow)">
							<animate
								attributeName="d"
								dur="3s"
								repeatCount="indefinite"
								values={generateAnimation(0.9, 0.6)}
							/>
						</path>
					</g>
				</svg>
				<div class="animate-blur absolute -inset-x-24 -bottom-16 -top-28">
					<svg width="100%" height="100%">
						<linearGradient id="backgroundGradient" gradientTransform="rotate(90)">
							<stop offset="0%" stop-color="#f87171">
								<animate
									attributeName="offset"
									dur="3s"
									repeatCount="indefinite"
									values="0; 0; -0.1; -0.2; -0.15; -0.05; -0.1; 0"
								/>
							</stop>
							<stop offset="50%" stop-color="#fdba74">
								<animate
									attributeName="offset"
									dur="6s"
									repeatCount="indefinite"
									values="0.6; 0.5; 0.35; 0.45; 0.25; 0.65; 0.5; 0.6"
									keyTimes="0; 0.1; 0.2; 0.3; 0.4; 0.6; 0.8; 1"
									calcMode="spline"
								/>
							</stop>
							<stop offset="100%" stop-color="#fef08a" />
						</linearGradient>

						<rect x="0" y="0" width="90%" height="90%" fill="url(#backgroundGradient)">
							<animate
								attributeName="x"
								dur="5s"
								repeatCount="indefinite"
								values="3%;2%;1%;3%;3%;1%;2%;3%;"
							/>
							<animate
								attributeName="y"
								dur="6s"
								repeatCount="indefinite"
								values="1%;2%;1%;2%;3%;1%;2%;1%;"
							/>
							<animate
								attributeName="width"
								dur="7s"
								repeatCount="indefinite"
								values="95%;100%;96%;97%;99%;97%;98%;95%;"
							/>
							<animate
								attributeName="height"
								dur="5.5s"
								repeatCount="indefinite"
								values="100%;98%;97%;99%;97%;99%;96%;100%;"
							/>
						</rect>
					</svg>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.animate-blur {
		/* Runs a blur & opacity animation on the element */
		animation: blur 10s ease-in-out infinite alternate;
	}

	@keyframes blur {
		0% {
			filter: blur(55px);
			opacity: 0.15;
		}
		10% {
			filter: blur(60px);
			opacity: 0.2;
		}
		20% {
			filter: blur(70px);
			opacity: 0.25;
		}
		30% {
			filter: blur(80px);
			opacity: 0.3;
		}
		40% {
			filter: blur(65px);
			opacity: 0.2;
		}
		50% {
			filter: blur(55px);
			opacity: 0.2;
		}
		60% {
			filter: blur(60px);
			opacity: 0.23;
		}
		70% {
			filter: blur(60px);
			opacity: 0.15;
		}
		80% {
			filter: blur(55px);
			opacity: 0.2;
		}
		90% {
			filter: blur(65px);
			opacity: 0.25;
		}
		100% {
			filter: blur(55px);
			opacity: 0.15;
		}
	}
</style>
