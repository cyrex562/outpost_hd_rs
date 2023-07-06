// ==================================================================================
// = NAS2D
// = Copyright © 2008 - 2020 New Age Software
// ==================================================================================
// = NAS2D is distributed under the terms of the zlib license. You are free to copy,
// = modify and distribute the software under the terms of the zlib license.
// =
// = Acknowledgment of your use of NAS2D is appreciated but is not required.
// ==================================================================================

#pragma once

#include "Delegate.h"
#include <vector>
#include <algorithm>


namespace NAS2D
{
	template <typename... Params>
	class SignalSource
	{
	public:
		using Source = SignalSource; // Restricted base interface for use in derived
		using DelegateType = Delegate<void(Params...)>;

	public:
		bool empty() const { return delegateList.empty(); }

		void clear() { delegateList.clear(); }

		void connect(DelegateType delegate)
		{
			const auto iterator = std::find(delegateList.begin(), delegateList.end(), delegate);
			if (iterator == delegateList.end())
			{
				delegateList.push_back(delegate);
			}
		}

		void disconnect(DelegateType delegate)
		{
			const auto iterator = std::find(delegateList.begin(), delegateList.end(), delegate);
			if (iterator != delegateList.end())
			{
				delegateList.erase(iterator);
			}
		}

	protected:
		std::vector<DelegateType> delegateList{};
	};
} // namespace
