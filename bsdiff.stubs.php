<?php

// Stubs for php-bsdiff

namespace {
    /**
     * Diff an "old" and a "new" file, returning a patch.
     *
     * @param string $old_file   The old file.
     * @param string $new_file   The new file.
     * @param string $patch_file The patch file that will be created.
     * @return bool Returns TRUE if succeeds, otherwise FALSE.
     */
    function bsdiff_diff(string $old_file, string $new_file, string $patch_file): bool {}

    /**
     * Apply a patch to an "old" file, returning the "new" file.
     *
     * @param string $old_file   The old file.
     * @param string $patch_file The patch file.
     * @param string $new_file   The new file that will be created.
     * @return bool Returns TRUE if succeeds, otherwise FALSE.
     */
    function bsdiff_patch(string $old_file, string $patch_file, string $new_file): bool {}
}
