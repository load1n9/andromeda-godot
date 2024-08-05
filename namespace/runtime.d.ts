export namespace console {
    /**
     *  log function logs a message to the console.
     *
     * @example
     * ```ts
     * console.log("Hello, World!");
     * ```
     */
    function log(message: string): void;
    
    /**
     * debug function logs a debug message to the console.
     *
     * @example
     * ```ts
     * console.debug("Hello, World!");
     */
    function debug(message: string): void;

    /**
     * warn function logs a warning message to the console.
     *
     * @example
     * ```ts
     * console.warn("Hello, World!");
     * ```
     */
    function warn(message: string): void;

    /**
     *  error function logs a warning message to the console.
     *
     * @example
     * ```ts
     * console.error("Hello, World!");
     * ```
     */
    function error(message: string): void;

    /**
     *  info function logs an info message to the console.
     *
     * @example
     * ```ts
     * console.info("Hello, World!");
     * ```
     */
    function info(message: string): void;   
}