const handleClick = onClick
	? (offset: number) =>
			({ photo, index, event }: ClickHandlerProps<TPhoto>) => {
				onClick({ photos: photosArray, index: offset + index, photo, event });
			}
	: undefined;
