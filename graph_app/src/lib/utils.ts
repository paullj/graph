const preventDefault = <T extends (...args: any[]) => any>(fn: T) => {
  return (event: Event) => {
    event.preventDefault();
    fn.call(event);
  };
};

const debounce = (func: (...args: any[]) => void, delay: number) => {
  let timeoutId: number | undefined;
  return (...args: any[]) => {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => func(...args), delay);
  };
};

export { preventDefault, debounce };
