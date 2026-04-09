/* should not generate diagnostics */

import { featureValue } from "./feature/value.ts";
import { nestedValue } from "./feature/nested/value.ts";
import { rootButton } from "@/shared/button.ts";
import { importedValue } from "#internal/imported.ts";
import { externalThing } from "pkg";
import("../shared/button.ts");
require("../shared/button.ts");

